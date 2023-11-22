pipeline {
  agent {
    docker {
      image 'node:20.9.0-alpine3.18'
      args '-p 3000:3000'
      reuseNode true
    }
  }
  environment {
    NODE_ENV = 'production'
  }
  stages {
    stage('Install') {
      steps {
        echo 'Installing..'
        sh 'yarn'
        echo 'Install Success'
      }
    }
    stage('Build') {
      steps {
        echo 'Building..'
        sh 'yarn build'
        echo 'Build Success'
      }
    }
    stage('Deploy') {
      when {
        branch 'main'
      }
      steps {
        echo 'Deploying..'
        input message: 'Finished using the web site? (Click "Proceed" to continue)'
        sh './jenkins/deploy.sh'
        echo 'Deploy Success'
      }
    }
  }
}