pipeline {
    agent any
    stages {
        stage("build") {
            steps {
                echo 'Building api...'
                sh 'cd api';
                sh 'cargo build';
                sh 'cd ..';
                echo 'Building Front-end...'
                sh 'cd app';
                sh 'npm i';
            }
        }
    } 
}