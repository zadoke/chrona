# How to deploy the CPTrainBot Backend on a Kubernetes Cluster

This guide will walk you through the process of deploying the CPTrainBot Backend on a Kubernetes cluster.

## Prerequisites

Before you begin, make sure you have the following:

-   A Kubernetes cluster, either self-hosted or managed
-   `kubectl` installed and configured to interact with your cluster
-   The backend deployment file (`cptrainbot-backend-deployment.yaml`)
-   The backend service file (`cptrainbot-backend-service.yaml`)

## Steps

1. **Apply the backend Deployment File**: Use `kubectl apply` to create the deployment defined in your backend file. Run the following command:
    ```
    kubectl apply -f cptrainbot-backend-deployment.yaml
    ```
2. **Verify the Deployment**: After applying your deployment file, use `kubectl get deployments` to verify that your deployment is running as expected.

3. **Apply the backend service file**: Use `kubectl apply` to create the service defined in your backend service file. Run the following command:
    ```
    kubectl apply -f cptrainbot-backend-service.yaml
    ```
4. **Verify the service**: Use `kubectl get services` to verify that your service is running and accessible.

5. **Access your deployed backend service**: At this point, your backend service should be running on your Kubernetes cluster. You can interact with it via any interfaces you've defined.

## Customization

The deployment and service files come with some default values, but you can modify them according to your needs:

-   **Backend Port**: By default, the backend service runs on port 3000. You can change this by modifying the `SERVER_PORT` environment variable in the backend deployment and the corresponding port in the backend service.
-   **Replicas**: The number of replicas for the backend is set by default (2). You can adjust this number based on your load requirements.
-   **Service Configuration**: The service file defines how traffic is routed to your backend pods. Feel free to customize this file according to your needs, such as configuring load balancing or setting up ingress rules.
