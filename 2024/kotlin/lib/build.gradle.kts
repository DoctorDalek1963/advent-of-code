plugins {
    application

    alias(libs.plugins.kotlin.jvm)

    // API and implementation separation
    `java-library`
}

repositories {
    mavenCentral()
}

dependencies {
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")
    testImplementation(libs.junit.jupiter.engine)

    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

application {
    if (hasProperty("launch")) {
        mainClass.set("com.github.doctordalek1963.aoc2024.${property("launch")}")
    }
}

tasks.named<Test>("test") {
    useJUnitPlatform()
    this.testLogging {
        outputs.upToDateWhen { false }
        this.showStandardStreams = true
    }
}
