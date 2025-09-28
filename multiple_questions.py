from scapy.all import DNS, DNSQR, IP, UDP, sr1

# Create the query with multiple questions
query = (
    IP(dst="127.0.0.1")
    / UDP(dport=2053)
    / DNS(
        rd=1,
        qd=[
            DNSQR(qname="example.com"),
            DNSQR(qname="test.example.com"),
            DNSQR(qname="another.test.example.com"),
        ],
    )
)

response = sr1(query)
print(response[DNS].an)
