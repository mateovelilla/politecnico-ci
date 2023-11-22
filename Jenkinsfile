pipeline {
    agent none
    stages {
        stage("build-backend") {
            agent {
                docker {
                    image 'rust:latest'
                }
            }
            steps {
                echo 'Building api...'
                sh 'ls';
                sh 'rustc --version';
            }
        }
        stage("build-frontend") {
            agent {
                docker {
                    image 'node:20.9.0-alpine3.18'
                }
            }
            steps {
                echo 'Building Front-end...'
                sh 'ls';
                sh 'node --version';
            }
        }
    } 
}