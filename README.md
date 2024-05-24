# kubectl-search

Search through config-maps and secrets.

## Usage

```shell
export KUBECONFIG=~/.kube/demo.kubeconfig

alias ks=kubectl-search

# Find all configmaps / secrets values which contains search mask

$ ks values [--search-secrets=false] "backup"

- ConfigMap: app-cm
  Keys:
  - 'BACKUP_SRV_HOST': 'app-backup-svc' 
  
- Secret: app-cm
  Keys:
  - 'BACKUP_USER': 'backup-user'
```