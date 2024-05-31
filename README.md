# kubectl-search

Search through config-maps and secrets (optional). Resolve vault paths to values (optional).

## Usage

```shell
export KUBECONFIG=~/.kube/demo.kubeconfig

# For Vault (optional)
#export VAULT_ADDR=~/.kube/demo.kubeconfig
#export VAULT_TOKEN=<token-here>

alias ks=kubectl-search

# Find all configmaps and secret values which contains search mask

# ks values [OPTIONS] <namespace> <search-mask>
$ ks values --secrets --vault apps "backup"

- config-map: app-cm
  - 'BACKUP_SRV_HOST': 'app-backup-svc' 
  
- config-map: another-app-cm
  - 'BACKUP_USER': 'app-backup-svc'
  
- secret: db-secret
  - 'BACKUP_USERNAME': '**********' # < each secret value is hidden
  
- secret: secret-with-vault-paths
  - 'BACKUP_USERNAME': '**********' # vault paths will be resolved if you use `--vault` flag
```

## How it works

Tool uses:
- `kubectl` to read data from kubernetes
- `vault` to read secrets from hashicorp vault

## Safety

- Tool doesn't use any write or delete commands inside cluster
- Tool doesn't log or print secret values. If you'd like to reveal secret values in stdout use `--unmask` flag.

## Troubleshooting

Logs are disabled by default you can turn it on with `--log-level=LEVEL` option:

```shell
$ ks --log-level=debug values --secrets --vault apps "backup"
```

## Roadmap

1. Mask secret values by mask: PASSWORD, TOKEN
2. Support cache