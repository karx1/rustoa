void setBuildStatus(String message, String state) {
  step([
      $class: "GitHubCommitStatusSetter",
      reposSource: [$class: "ManuallyEnteredRepositorySource", url: "https://github.com/karx1/rustoa"],
      contextSource: [$class: "ManuallyEnteredCommitContextSource", context: "Jenkins CI"],
      errorHandlers: [[$class: "ChangingBuildStatusErrorHandler", result: "UNSTABLE"]],
      statusResultSource: [ $class: "ConditionalStatusResultSource", results: [[$class: "AnyBuildResult", message: message, state: state]] ]
  ]);
}


pipeline {
	agent {
		docker {
			image 'rust:latest'
			args '-v $HOME/rust/rustoa/target:/target -v $HOME/rust/rustoa/cargo:/root/.cargo'
		}
	}
	stages {
		stage('Test') {
			steps {
				setBuildStatus("Build pending", "PENDING");
				withCredentials([string(credentialsId: 'toa-key', variable: 'API_KEY')]) {
					sh 'cargo test --target-dir /target'
				}
			}
		}
		stage('Nightly') {
			agent {
				docker { image 'rustlang/rust:nightly' }
			}
			steps {
				withCredentials([string(credentialsId: 'toa-key', variable: 'API_KEY')]) {
					sh 'cargo test || true'
				}
			}
		}
	}
	post {
		success {
			withCredentials([string(credentialsId: 'cargo-token', variable: 'TOKEN')]) {
				sh 'cargo login $TOKEN || true'
				sh 'cargo publish --target-dir /target || true'
			}
			withCredentials([string(credentialsId: 'karx-discord-webhook', variable: 'DISCORD')]) {
				discordSend webhookURL: DISCORD, title: "Jenkins Pipeline Build", link: env.BUILD_URL, result: currentBuild.currentResult, footer: "rustoa", description: "Build SUCCESS"
			}			
setBuildStatus("Build succeeded", "SUCCESS");
		}
		failure {
		    withCredentials([string(credentialsId: 'karx-discord-webhook', variable: 'DISCORD')]) {
			    discordSend webhookURL: DISCORD, title: "Jenkins Pipeline Build", link: env.BUILD_URL, result: currentBuild.currentResult, footer: "rustoa", description: "Build FAILURE"
			}
			setBuildStatus("Build failed", "FAILURE");
		}
	}
}
