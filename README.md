# kubectl-search

Search through config-maps.

## Usage

```shell
export KUBECONFIG=~/.kube/demo.kubeconfig

alias ks=kubectl-search

# Find all configmaps values which contains search mask

$ ks values "backup"

- ConfigMap: app-cm
  Keys:
  - 'BACKUP_SRV_HOST': 'app-backup-svc' 
  
- ConfigMap: another-app-cm
  Keys:
  - 'BACKUP_USER': 'app-backup-svc'

```

## How it works

Tool uses `kubectl` to read data from kubernetes.

## Safety

Tool doesn't use any write or delete commands inside cluster.

## Roadmap

1. Search values in ConfigMaps
2. Search values in Secrets