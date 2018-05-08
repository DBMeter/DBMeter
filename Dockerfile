FROM ubuntu

COPY target/release/dbmeter /usr/bin/dbmeter

CMD ["/usr/bin/dbmeter"]