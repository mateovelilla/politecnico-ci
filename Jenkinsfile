pipeline {
    agent any
    stages {
        stage("build-backend") {
            agent {
                docker {
                    image 'rust:latest'
                    args '-p 8082:8082'
                }
            }
            steps {
                echo 'Building api...'
                sh 'cd api';
                sh 'cargo build';
            }
        }
        stage("build-frontend") {
            agent {
                docker {
                    image 'node:20.9.0-alpine3.18'
                }
            }
            steps {
                sh 'cd ..';
                echo 'Building Front-end...'
                sh 'cd app';
                sh 'npm i';
                sh 'npm run build';
            }
        }
    } 
}