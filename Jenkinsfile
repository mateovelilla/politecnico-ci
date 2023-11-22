pipeline {
  agent {
    docker {
      image 'node:18-alpine'
    }
  }
  stages {
    stage('Build front-end') {
      steps {
        sh 'ls'
        sh 'npm install'
      }
    }
  }
}