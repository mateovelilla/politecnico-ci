pipeline {
  agent {
    docker {
      image: 'node:18-alpine'
    }
  }
  stages {
    stage('Build front-end') {
      sh 'npm install'
    }
  }
}