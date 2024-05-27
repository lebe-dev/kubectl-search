# kubectl-search

Search through config-maps.

## Usage

```shell
export KUBECONFIG=~/.kube/demo.kubeconfig

alias ks=kubectl-search

# Find all configmaps values which contains search mask

# ks values <namespace> <search-mask>
$ ks values apps "backup"

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

1. Search values in Secrets
2. Search values in Vault secret values
3. Support cache