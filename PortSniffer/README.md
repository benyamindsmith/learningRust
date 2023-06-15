# PortSniffer

__Some notes:__ 

* To create a new project, we need to run in the terminal the following script: 

		`cargo new ip_sniffer --bin`

* This creates a new project called `ip_sniffer`. Since we want this to be a binary we pass the `--bin` flag. 

* These are the inputs for what the command line tool should look like:
		
		- `ip_sniffer.exe -h`:  a flag that will present a help screen
		
		- `ip_sniffer.exe -j 100 192.168.1.1`: a flag that will allow te user to set the number of threads that he wants to use. 

		- `ip_sniffer.exe 192.168.1.1` this just calls the tool and buys it on an ip address and will use the default set number of threads.  


