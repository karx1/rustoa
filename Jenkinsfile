pipeline {
	agent { docker { image 'rust:latest' } }
	stages {
		stage('Test') {
			steps {
				sh 'cargo check'
				sh 'cargo build'
				sh 'cargo test'
			}
		}
	}
}
