pipeline {
    agent none
    stages {
        stage("build-backend") {
            agent {
                docker {
                    image 'rust:latest'
                    reuseNode true
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
                    reuseNode true
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