pipeline {
  agent {
    docker {
      image: 'node:18-alpine'
    }
  }
  stages {
    stage('Deploy') {
      sh 'docker compose up'
    }
  }
}