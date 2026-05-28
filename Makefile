.PHONY: up down build logs \
        k8s-up k8s-down k8s-build k8s-load k8s-apply k8s-status k8s-cluster k8s-ingress

up:
	docker compose up --build -d

down:
	docker compose down

k8s-up: k8s-cluster k8s-ingress k8s-build k8s-load k8s-apply

k8s-cluster:
	kind create cluster --name chat --config kind-config.yaml

k8s-ingress:
	kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
	kubectl rollout status deployment/ingress-nginx-controller -n ingress-nginx --timeout=90s

k8s-build:
	docker build -t chat-backend:latest ./backend
	docker build -t chat-frontend:latest ./frontend

k8s-load:
	kind load docker-image chat-backend:latest --name chat
	kind load docker-image chat-frontend:latest --name chat

k8s-apply:
	kubectl apply -f k8s/

k8s-status:
	kubectl get pods
	kubectl get ingress

k8s-down:
	kind delete cluster --name chat