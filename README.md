**IP Sniffer built using Rust**

This is a CLI tool built using Rust that displays a list of available ports for a given IP address

Commands:<br>
* -a, --address <ARG>  The address you want to sniff. Must be a valid IPv4 address. Falls back to
                     127.0.0.1<br>
* -s, --start <ARG>    The start port for the sniffer (must be greater than 0)<br>
* -e, --end <ARG>      The end port for the sniffer (must be less than or equal to 65535)<br>
* -h, --help           Prints help information<br>
