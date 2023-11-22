pipeline {
    agent any
    stages {
        stages("build") {
            steps {
                echo 'Building api...'
                sh 'cd api';
                sh 'cd cargo build';
                sh 'cd ..';
                echo 'Building Front-end...'
                sh 'cd app';
                sh 'npm i';
            }
        }
    } 
}