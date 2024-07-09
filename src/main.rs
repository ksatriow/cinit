use clap::{Command};
use std::fs::File;
use std::io::Write;

fn main() {
    let matches = Command::new("cinit")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Initializes CI/CD configuration files")
        .subcommand(Command::new("gitlabci")
            .about("Initializes a GitLab CI configuration"))
        .subcommand(Command::new("github")
            .about("Initializes a GitHub Actions configuration"))
        .subcommand(Command::new("bitbucket")
            .about("Initializes a Bitbucket Pipeline configuration"))
        .subcommand(Command::new("jenkins")
            .about("Initializes a Jenkinsfile configuration"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("gitlabci") {
        init_gitlabci();
    } else if let Some(_) = matches.subcommand_matches("github") {
        init_github_actions();
    } else if let Some(_) = matches.subcommand_matches("bitbucket") {
        init_bitbucket_pipeline();
    } else if let Some(_) = matches.subcommand_matches("jenkins") {
        init_jenkinsfile();
    }
}

fn init_gitlabci() {
    let gitlab_ci_content = r#"
stages:
  - build
  - test
  - deploy

build:
  stage: build
  script:
    - echo "Building..."

test:
  stage: test
  script:
    - echo "Testing..."

deploy:
  stage: deploy
  script:
    - echo "Deploying..."
"#;

    write_to_file(".gitlab-ci.yml", gitlab_ci_content);
}

fn init_github_actions() {
    let github_actions_content = r#"
name: CI

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v2
    - name: Run a one-line script
      run: echo Hello, world!
    - name: Run a multi-line script
      run: |
        echo Add other actions to build,
        echo test, and deploy your project.
"#;

    std::fs::create_dir_all(".github/workflows").expect("Could not create directories");
    write_to_file(".github/workflows/ci.yml", github_actions_content);
}

fn init_bitbucket_pipeline() {
    let bitbucket_pipeline_content = r#"
pipelines:
  default:
    - step:
        name: Build and Test
        script:
          - echo "Building..."
          - echo "Testing..."
"#;

    write_to_file("bitbucket-pipelines.yml", bitbucket_pipeline_content);
}

fn init_jenkinsfile() {
    let jenkinsfile_content = r#"
pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                echo 'Building...'
            }
        }
        stage('Test') {
            steps {
                echo 'Testing...'
            }
        }
        stage('Deploy') {
            steps {
                echo 'Deploying...'
            }
        }
    }
}
"#;

    write_to_file("Jenkinsfile", jenkinsfile_content);
}

fn write_to_file(file_name: &str, content: &str) {
    let mut file = File::create(file_name).expect("Could not create file");
    file.write_all(content.as_bytes()).expect("Could not write to file");
    println!("{} created successfully.", file_name);
}
