# ipr

This is a tool that allows you to get information related to your IP(v4) address.

## Useage

### Options

| options  | about                               |
| -------- | ----------------------------------- |
| ip       | IPv4 address                        |
| net_mask | The bit numbers of the network part |

### Example

```
ipr --ip 192.168.32.12 --net_mask 12
ipr -i 192.168.32.12 -n 12
```

### Result

```
ip                 192.168.32.12
cidr               192.168.32.12/12
subnet mask        255.240.0.0
network address    192.160.0.0
broadcast address  192.175.255.255
ip address range   192.160.0.0 ~ 192.175.255.255
ip address number  1048576
ip(bit)            11000000101010000010000000001100
subnet mask(bit)   11111111111100000000000000000000
```

## License

MIT
