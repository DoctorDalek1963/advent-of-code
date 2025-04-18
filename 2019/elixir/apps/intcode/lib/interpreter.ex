defmodule Intcode.Interpreter do
  @moduledoc """
  Interpret the [`Intcode`](https://adventofcode.com/2019/day/2) language built
  for [AOC 2019](https://adventofcode.com/2019).

  `Intcode` is a system of bytecode which operates on a list of integers. This
  interpreter is implemented in a way that makes use of message passing to
  interface with the user.

  ## Messages

  When the interpreter encounters an input instruction, it will send
  `:awaiting_input` to the user. It will then wait for a response, which should
  be of the form `{:input, value}`, where `value` is an integer.

  When it encounters an output instruction, it will send `{:output, value}` to
  the user, where `value` is an integer.

  When it encounters a halt instruction, it will send `{:halted, memory}`,
  where `memory` is a snapshot of the memory when the interpreter halted.

  If the interpreter runs into an error, it will send `{:error, message}`,
  where `message` is a string.

  ## Instructions

  Most instructions have *parameter modes* for their values. The last two
  digits of the instruction in base 10 are the opcode. This encodes the main
  content of the instruction and tells the interpreter what to do. The other
  digits in base 10 encode the parameter modes of the instructions.

  ### Parameter modes

  *Position mode* is represented by 0. In this mode, the parameter is a memory
  address and the interpreter will fetch the necessary value from that address.

  *Immediate mode* is represented by 1. In this mode, the parameter is a
  literal value and the interpreter will use the literal value of the parameter
  for the operation

  *Relative mode* is represented by 2. In this mode, the parameter is an offset
  from the *relative base* and the interpreter will fetch the necessary value
  from the relative base, plus the offset.

  ### Opcodes

  `[xy01, $r1, $r2, $r3]` => `$r3 = $r1 + $r2`. `y` is the parameter mode of
  `$r1` and `x` is the parameter mode of `$r2`. `$r3` is always in position mode.

  `[xy02, $r1, $r2, $r3]` => `$r3 = $r1 * $r2`. `y` is the parameter mode of
  `$r1` and `x` is the parameter mode of `$r2`. `$r3` is always in position mode.

  `[03, $r1]` => Send `:awaiting_input`, expect to receive `{:input, value}`,
  then store `value` in address `$r1`.

  `[x04, $r1]` => Send `{:output, value}` to the user, where `value` is
  determined by `$r1` and `x` is its parameter mode.

  `[xy05, $r1, $r2]` => If `$r1` is non-zero, jump to the address specified by
  `$r2.` `y` is the parameter mode of `$r1` and `x` is the parameter mode of `$r2`.

  `[xy06, $r1, $r2]` => If `$r1` is zero, jump to the address specified by
  `$r2.` `y` is the parameter mode of `$r1` and `x` is the parameter mode of `$r2`.

  `[xy07, $r1, $r2, $r3]` => If `$r1 < $r2`, then store a 1 at the address
  `$r3`. Otherwise, store a 0 there. `y` is the parameter mode of `$r1` and `x`
  is the parameter mode of `$r2`.

  `[xy08, $r1, $r2, $r3]` => If `$r1 == $r2`, then store a 1 at the address
  `$r3`. Otherwise, store a 0 there. `y` is the parameter mode of `$r1` and `x`
  is the parameter mode of `$r2`.

  `[x09, $r1]` => Adjust the relative base by the value of `$r1`. `x` is the
  parameter mode of `$r1`.

  `[99]` => Halt immediately, sending the current state of the interpreter's
  memory to the user as `{:halted, memory}`.
  """

  @typedoc """
  A snapshot of the interpreter's memory.
  """
  @type memory() :: [integer()]

  @typedoc """
  A parameter mode. See the module docs for details.
  """
  @type parameter_mode() :: :position | :immediate | :relative

  @typedoc """
  A parameter for an instruction, bundled with its mode.
  """
  @type parameter() :: {parameter_mode(), integer()}

  @typedoc """
  The mode of an address. Either a literal address in memory, or an offset from
  the *relative base*.
  """
  @type address_mode() :: :position | :relative

  @typedoc """
  An alias used for an instruction parameter which is always an address.
  """
  @type address() :: {address_mode(), integer()}

  @typedoc """
  An instruction for the interpreter.
  """
  @type instruction() ::
          {:add, parameter(), parameter(), address()}
          | {:multiply, parameter(), parameter(), address()}
          | {:input, address()}
          | {:output, parameter()}
          | {:jump_if_true, parameter(), parameter()}
          | {:jump_if_false, parameter(), parameter()}
          | {:less_than, parameter(), parameter(), address()}
          | {:equals, parameter(), parameter(), address()}
          | {:adjust_relative_base, parameter()}
          | :halt

  @doc """
  Interpret the given bytecode.

  The first argument is the PID of the user. The interpreter will send messages
  to the user for I/O operations. See the module docs for more details.

  This is the only function that is allowed to send `{:halted, memory}` to the
  user, and when it does, it will immediately kill this interpreter's process.
  It will also kill this interpreter's process immediately after sending an
  `{:error, message}` signal to the user.
  """
  @spec interpret(pid(), memory(), integer(), integer()) :: true
  def interpret(user_pid, memory, relative_base \\ 0, program_counter \\ 0)
      when is_pid(user_pid) and is_list(memory) and is_integer(relative_base) and
             is_integer(program_counter) do
    case parse_instruction(memory, program_counter) do
      {:error, msg} ->
        send(user_pid, {:error, msg})
        Process.exit(self(), :kill)

      instruction ->
        case new_pc(instruction, memory, relative_base, program_counter) do
          :halt ->
            send(user_pid, {:halted, memory})
            Process.exit(self(), :kill)

          new_pc when is_integer(new_pc) ->
            {new_memory, new_relative_base} =
              apply_instruction(memory, relative_base, user_pid, instruction)

            interpret(user_pid, new_memory, new_relative_base, new_pc)
        end
    end
  end

  @doc """
  Convert a number to a parameter mode.

  ## Examples
      iex> to_param_mode(0)
      :position
      iex> to_param_mode(1)
      :immediate
      iex> to_param_mode(2)
      :relative
  """
  @spec to_param_mode(integer()) :: parameter_mode()
  def to_param_mode(num) do
    case num do
      0 -> :position
      1 -> :immediate
      2 -> :relative
    end
  end

  @doc """
  Parse the instruction in memory at the program counter.

  ## Examples
      iex> parse_instruction([1, 0, 0, 0], 0)
      {:add, {:position, 0}, {:position, 0}, {:position, 0}}
      iex> parse_instruction([1002, 10, 3, 0], 0)
      {:multiply, {:position, 10}, {:immediate, 3}, {:position, 0}}
      iex> parse_instruction([1, 0, 0, 0, 102, 10, 3, 0, 99], 4)
      {:multiply, {:immediate, 10}, {:position, 3}, {:position, 0}}
      iex> parse_instruction([1, 0, 0, 0, 1002, 10, 3, 0, 99], 8)
      :halt
      iex> parse_instruction([], 0)
      {:error, "program counter out of bounds (address 0)"}
  """
  @spec parse_instruction(memory(), integer()) :: instruction() | {:error, String.t()}
  def parse_instruction(memory, program_counter) do
    case Enum.at(memory, program_counter) do
      nil ->
        {:error, "program counter out of bounds (address #{program_counter})"}

      byte when is_integer(byte) ->
        opcode = rem(byte, 100)
        mode1 = to_param_mode(rem(div(byte, 100), 10))
        mode2 = to_param_mode(rem(div(byte, 1000), 10))
        mode3 = to_param_mode(rem(div(byte, 10_000), 10))

        get_mem = fn offset ->
          case Enum.at(memory, program_counter + offset) do
            nil ->
              {
                :error,
                "failed to find instruction parameter at address #{program_counter + offset}"
              }

            x when is_integer(x) ->
              x
          end
        end

        case opcode do
          1 ->
            {
              :add,
              {mode1, get_mem.(1)},
              {mode2, get_mem.(2)},
              {mode3, get_mem.(3)}
            }

          2 ->
            {
              :multiply,
              {mode1, get_mem.(1)},
              {mode2, get_mem.(2)},
              {mode3, get_mem.(3)}
            }

          3 ->
            {
              :input,
              {mode1, get_mem.(1)}
            }

          4 ->
            {
              :output,
              {mode1, get_mem.(1)}
            }

          5 ->
            {
              :jump_if_true,
              {mode1, get_mem.(1)},
              {mode2, get_mem.(2)}
            }

          6 ->
            {
              :jump_if_false,
              {mode1, get_mem.(1)},
              {mode2, get_mem.(2)}
            }

          7 ->
            {
              :less_than,
              {mode1, get_mem.(1)},
              {mode2, get_mem.(2)},
              {mode3, get_mem.(3)}
            }

          8 ->
            {
              :equals,
              {mode1, get_mem.(1)},
              {mode2, get_mem.(2)},
              {mode3, get_mem.(3)}
            }

          9 ->
            {:adjust_relative_base, {mode1, get_mem.(1)}}

          99 ->
            :halt

          _ ->
            {:error, "unrecognised opcode #{opcode} at address #{program_counter}"}
        end
    end
  end

  @doc """
  Perform the instruction on the memory and return the new memory after
  applying the effect of the instruction.

  This function has no special handling for instructions like `:halt`. It will
  just return the memory unchanged and *will not* send the `{:halted, memory}`
  message.

  In constrast, it will send the `:awaiting_input` and `{:output, value}`
  messages to the user.

  ## Examples
      iex> user_pid = nil # Should be the PID of the process that will handle I/O
      iex> apply_instruction(
      ...>   [1, 2, 3, 4],
      ...>   0,
      ...>   user_pid,
      ...>   {:add, {:position, 1}, {:immediate, 6}, {:position, 3}}
      ...> )
      {[1, 2, 3, 8], 0}
      iex> apply_instruction(
      ...>   [1, 2, 3, 4],
      ...>   0,
      ...>   user_pid,
      ...>   {:multiply, {:immediate, 10}, {:position, 1}, {:position, 2}}
      ...> )
      {[1, 2, 20, 4], 0}
      iex> apply_instruction([1, 2, 3, 4], 0, user_pid, :halt)
      {[1, 2, 3, 4], 0}
      iex> apply_instruction(
      ...>   [1, 2, 3, 4],
      ...>   0,
      ...>   user_pid,
      ...>   {:adjust_relative_base, {:immediate, 1}}
      ...> )
      {[1, 2, 3, 4], 1}
      iex> apply_instruction(
      ...>   [1, 2, 3, 4],
      ...>   1,
      ...>   user_pid,
      ...>   {:adjust_relative_base, {:relative, 1}}
      ...> )
      {[1, 2, 3, 4], 4}
  """
  @spec apply_instruction(memory(), integer(), pid(), instruction()) :: {memory(), integer()}
  def apply_instruction(memory, relative_base, user_pid, instruction) do
    case instruction do
      {:add, r1, r2, r3} ->
        {List.replace_at(
           memory,
           get_addr(relative_base, r3),
           get_param(memory, relative_base, r1) + get_param(memory, relative_base, r2)
         ), relative_base}

      {:multiply, r1, r2, r3} ->
        {List.replace_at(
           memory,
           get_addr(relative_base, r3),
           get_param(memory, relative_base, r1) * get_param(memory, relative_base, r2)
         ), relative_base}

      {:input, r1} ->
        send(user_pid, :awaiting_input)

        receive do
          {:input, value} when is_integer(value) ->
            {List.replace_at(memory, get_addr(relative_base, r1), value), relative_base}
        end

      {:output, r1} ->
        send(user_pid, {:output, get_param(memory, relative_base, r1)})
        {memory, relative_base}

      # Jumps don't affect memory. They're handled in `new_pc/4`
      {:jump_if_true, _, _} ->
        {memory, relative_base}

      {:jump_if_false, _, _} ->
        {memory, relative_base}

      {:less_than, r1, r2, r3} ->
        if get_param(memory, relative_base, r1) < get_param(memory, relative_base, r2) do
          {List.replace_at(memory, get_addr(relative_base, r3), 1), relative_base}
        else
          {List.replace_at(memory, get_addr(relative_base, r3), 0), relative_base}
        end

      {:equals, r1, r2, r3} ->
        if get_param(memory, relative_base, r1) === get_param(memory, relative_base, r2) do
          {List.replace_at(memory, get_addr(relative_base, r3), 1), relative_base}
        else
          {List.replace_at(memory, get_addr(relative_base, r3), 0), relative_base}
        end

      {:adjust_relative_base, r1} ->
        {memory, relative_base + get_param(memory, relative_base, r1)}

      :halt ->
        {memory, relative_base}
    end
  end

  @doc """
  Work out the new program counter based on the old one.

  If the instruction is `:halt`, this function will return `:halt`.
  """
  @spec new_pc(instruction(), memory(), integer(), integer()) :: integer() | :halt
  def new_pc(instruction, memory, relative_base, old_pc) do
    case instruction do
      {:jump_if_true, r1, r2} ->
        cond = get_param(memory, relative_base, r1)

        if is_integer(cond) and cond !== 0 do
          get_param(memory, relative_base, r2)
        else
          old_pc + 3
        end

      {:jump_if_false, r1, r2} ->
        cond = get_param(memory, relative_base, r1)

        if is_integer(cond) and cond === 0 do
          get_param(memory, relative_base, r2)
        else
          old_pc + 3
        end

      :halt ->
        :halt

      {opcode, _} when is_atom(opcode) ->
        old_pc + 2

      {opcode, _, _} when is_atom(opcode) ->
        old_pc + 3

      {opcode, _, _, _} when is_atom(opcode) ->
        old_pc + 4
    end
  end

  @doc """
  Convert a `t:parameter/0` into a proper value by fetching it from memory if necessary.

  ## Examples
      iex> get_param([1, 2, 3, 4], 0, {:position, 2})
      3
      iex> get_param([1, 2, 3, 4], 0, {:immediate, 5})
      5
      iex> get_param([1, 2, 3, 4], 0, {:position, 5})
      {:error, "attempted to fetch from out-of-bounds memory (address 5)"}
      iex> get_param([1, 2, 3, 4], 0, {:relative, 2})
      3
      iex> get_param([1, 2, 3, 4], 1, {:relative, 2})
      4
  """
  @spec get_param(memory(), integer(), parameter()) :: integer() | {:error, String.t()}
  def get_param(memory, relative_base, {mode, value}) do
    case mode do
      :position ->
        case Enum.at(memory, value) do
          nil -> {:error, "attempted to fetch from out-of-bounds memory (address #{value})"}
          x when is_integer(x) -> x
        end

      :immediate ->
        value

      :relative ->
        case Enum.at(memory, relative_base + value) do
          nil ->
            {:error,
             "attempted to fetch from out-of-bounds memory (address #{relative_base + value})"}

          x when is_integer(x) ->
            x
        end
    end
  end

  @doc """
  Get the address for this value.

  If the mode is `:position`, then the address is the same as the value. If the
  mode is `:relative`, then the address is `relative_base + value`.
  """
  @spec get_addr(integer(), address()) :: integer()
  def get_addr(relative_base, {mode, value}) do
    case mode do
      :position -> value
      :relative -> relative_base + value
    end
  end
end
