# Generate Email Security DNS Records

Generate Email Security DNS Records for DKIM, SPF and DMARC.

### Example run
```
./records-gen example.com 
Add the following DNS records to your domain in order to enable DKIM, SPF and DMARC: example.com

 TYPE   NAME                         CONTENT 
 TXT    example.com                  v=spf1 -all 
 TXT    _dmarc.example.com           v=DMARC1; p=reject; rua=mailto:postmaster@example.com; ruf=mailto:postmaster@example.com 
 TXT    s1r._domainkey.example.com   v=DKIM1; k=rsa; h=sha256; p=MIIBCgKCAQEAwkHYFBOunpjkTVI+706KnYAjTota7ws824czVIDJMpFX9JjaYou1G++vN68A0YUV898xhgqMuIpBfm7pj+C0Q4biS7RCtAPCpx/i4+yYkB7gN/ikawokBQif++7+Fsfh4Y+KTGQbvDw2T601j1fQrb87SPQMqsjDNmOmhHuOIRke9OPJ3jGDLJVMYU/bky2VoPRAjkC7WMaVncDF54rARLOH3WTDIXjpWTWFa3dwKMFFY8WcuEVWaQAHweKZFCGhkVO/t5tPr/QlDHFY6Zb+eymGIlafbtQDmlXmc9OCuxQgffu+0aAYwgfoI0Wr6eA2ydjIaasVEtMjbz+abuT7xwIDAQAB 

Save the following DKIM Private/Public keys for your SMTP Server
Private Key : You must enter this key in your DKIM signer. It must be kept secret, as anyone with access to it can stamp tokens pretending to be you.

-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAwkHYFBOunpjkTVI+706KnYAjTota7ws824czVIDJMpFX9Jja
You1G++vN68A0YUV898xhgqMuIpBfm7pj+C0Q4biS7RCtAPCpx/i4+yYkB7gN/ik
awokBQif++7+Fsfh4Y+KTGQbvDw2T601j1fQrb87SPQMqsjDNmOmhHuOIRke9OPJ
3jGDLJVMYU/bky2VoPRAjkC7WMaVncDF54rARLOH3WTDIXjpWTWFa3dwKMFFY8Wc
uEVWaQAHweKZFCGhkVO/t5tPr/QlDHFY6Zb+eymGIlafbtQDmlXmc9OCuxQgffu+
0aAYwgfoI0Wr6eA2ydjIaasVEtMjbz+abuT7xwIDAQABAoIBAE+OV8ghJ2qcgyOH
g27OY8aNK+iG+f7cQ1hEzqjU9ZFYJQVvdEDXppyCVOghK9yi/JKb3dRfjbImLRxy
oATkqe7d1t/aO1cuExIO5QUjaHfdrUY+9ldAF/BXz+2Fu1npEoEmICWY+iLlJfkL
ntvNwxDlOIpwKJ/pCMnFlfS8BUFYKK7J+pyJTZeTzAQkOn44Jn8K60JuhmRTHDBf
BTcxhiUBZLZsgO9WRdbi9tvM00SP//FA/0alm59VKqIuqUdSjIkB36x+IPKJ1Icm
8DUgvwZBAEVj7o6gQl7HtzTYhhxJacV+xps7tPKYskYJR5rXYpz2C+XzmJF8aocq
ltv1AwECgYEA+K2UOwlCrvNKLk1PqlgdixOXiiVoST3S4NmJKlBJIWIdMvyaSBUh
0pqZbKu8eJNGBgTwGeWh5fjj9bPZ1YtDiMLC+zocw0qwd5Pwfq5JSh2T/+HVIJyl
csktnWqpIbXU12kSIda3xrDA749WnyapFqqCy9yLLs0W5kdXPajK6qTb8bKKxXZs
z1l8y5+HgZ7DczkRFLYJK0V2A4L+qgjxTEBd3bECgYEAhByrUEXWMF29/iVCeov/
fgB1lIXVBnwaySuojFtEzsByJMFAMaVQe17u5zYCylygm3zRkPOe9bnBYgOIE0Tp
S8b4n3OVPwe1dtDbuT1ALeajO/FWWFKQ+RZZMMZG6PtKnyIseYLiWB72r4wL9JLX
LPx2hWss2vcy8MelOAlIpvcCgYBb60nIRF+5X8cl9Sv+oE5s2Oxt9NVzntPGIFlO
P/AXHKytOoAIIUqsgOWkrVqb4mF/GkwxkwSV5CI5JmzYjaaYSoQ0a50vSAsexTWE
wyFiQQKBgQCsTsTi7wvDwLm09k7oNMm25JEyDQphZ+NmaOAsREsdzSWg5gDXhM58
QOswuA5j+I3aEdxn1mpAw0lzhaB9ct3Vf4Y6HuTnOewhdOwsRlzVDhigDrtPc2Zw
D8x32Oy7duUrB9QazxN0IIPkzeI70H9lMzhIjRo5MeilIhKK2KMzZQ==
-----END RSA PRIVATE KEY-----

Public Key : This is the public key in the original raw "X509" format. It's not usable in DNS directly, but it might be useful for something else.

-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAwkHYFBOunpjkTVI+706KnYAjTota7ws824czVIDJMpFX9JjaYou1
G++vN68A0YUV898xhgqMuIpBfm7pj+C0Q4biS7RCtAPCpx/i4+yYkB7gN/ikawok
BQif++7+Fsfh4Y+KTGQbvDw2T601j1fQrb87SPQMqsjDNmOmhHuOIRke9OPJ3jGD
LJVMYU/bky2VoPRAjkC7WMaVncDF54rARLOH3WTDIXjpWTWFa3dwKMFFY8WcuEVW
aQAHweKZFCGhkVO/t5tPr/QlDHFY6Zb+eymGIlafbtQDmlXmc9OCuxQgffu+0aAY
wgfoI0Wr6eA2ydjIaasVEtMjbz+abuT7xwIDAQAB
-----END RSA PUBLIC KEY-----

```
- With a custom selector
./records-gen example.com --selector t1
```
Add the following DNS records to your domain in order to enable DKIM, SPF and DMARC: example.com

TYPE   NAME                         CONTENT
TXT    example.com                  v=spf1 -all
TXT    _dmarc.example.com           v=DMARC1; p=reject; rua=mailto:postmaster@example.com; ruf=mailto:postmaster@example.com
TXT    t1r._domainkey.example.com   v=DKIM1; k=rsa; h=sha256; p=MIIBCgKCAQEAwkHYFBOunpjkTVI+706KnYAjTota7ws824czVIDJMpFX9JjaYou1G++vN68A0YUV898xhgqMuIpBfm7pj+C0Q4biS7RCtAPCpx/i4+yYkB7gN/ikawokBQif++7+Fsfh4Y+KTGQbvDw2T601j1fQrb87SPQMqsjDNmOmhHuOIRke9OPJ3jGDLJVMYU/bky2VoPRAjkC7WMaVncDF54rARLOH3WTDIXjpWTWFa3dwKMFFY8WcuEVWaQAHweKZFCGhkVO/t5tPr/QlDHFY6Zb+eymGIlafbtQDmlXmc9OCuxQgffu+0aAYwgfoI0Wr6eA2ydjIaasVEtMjbz+abuT7xwIDAQAB

Save the following DKIM Private/Public keys for your SMTP Server
Private Key : You must enter this key in your DKIM signer. It must be kept secret, as anyone with access to it can stamp tokens pretending to be you.

-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAwkHYFBOunpjkTVI+706KnYAjTota7ws824czVIDJMpFX9Jja
You1G++vN68A0YUV898xhgqMuIpBfm7pj+C0Q4biS7RCtAPCpx/i4+yYkB7gN/ik
awokBQif++7+Fsfh4Y+KTGQbvDw2T601j1fQrb87SPQMqsjDNmOmhHuOIRke9OPJ
3jGDLJVMYU/bky2VoPRAjkC7WMaVncDF54rARLOH3WTDIXjpWTWFa3dwKMFFY8Wc
g27OY8aNK+iG+f7cQ1hEzqjU9ZFYJQVvdEDXppyCVOghK9yi/JKb3dRfjbImLRxy
oATkqe7d1t/aO1cuExIO5QUjaHfdrUY+9ldAF/BXz+2Fu1npEoEmICWY+iLlJfkL
ntvNwxDlOIpwKJ/pCMnFlfS8BUFYKK7J+pyJTZeTzAQkOn44Jn8K60JuhmRTHDBf
BTcxhiUBZLZsgO9WRdbi9tvM00SP//FA/0alm59VKqIuqUdSjIkB36x+IPKJ1Icm
8DUgvwZBAEVj7o6gQl7HtzTYhhxJacV+xps7tPKYskYJR5rXYpz2C+XzmJF8aocq
96DXzb5TTyXaBEKnI/xSrxppa/eD/qeDmMyOTs7d6O8k0ah6ISsAeMCmXfyeryJP
XOHN6+TK5oY2vKH+Ej6ordQB14k0pIuVFwxTsBpecIMKJ2o5kc0M9vcCgYEAx/oQ
0pqZbKu8eJNGBgTwGeWh5fjj9bPZ1YtDiMLC+zocw0qwd5Pwfq5JSh2T/+HVIJyl
csktnWqpIbXU12kSIda3xrDA749WnyapFqqCy9yLLs0W5kdXPajK6qTb8bKKxXZs
z1l8y5+HgZ7DczkRFLYJK0V2A4L+qgjxTEBd3bECgYEAhByrUEXWMF29/iVCeov/
fgB1lIXVBnwaySuojFtEzsByJMFAMaVQe17u5zYCylygm3zRkPOe9bnBYgOIE0Tp
S8b4n3OVPwe1dtDbuT1ALeajO/FWWFKQ+RZZMMZG6PtKnyIseYLiWB72r4wL9JLX
LPx2hWss2vcy8MelOAlIpvcCgYBb60nIRF+5X8cl9Sv+oE5s2Oxt9NVzntPGIFlO
h5dwOun1qmMq08l7sUjm1Gmu+HMqplY2SNq6vMg5Qocu7JD1FOksXxqQAubiCLc4
P/AXHKytOoAIIUqsgOWkrVqb4mF/GkwxkwSV5CI5JmzYjaaYSoQ0a50vSAsexTWE
wyFiQQKBgQCsTsTi7wvDwLm09k7oNMm25JEyDQphZ+NmaOAsREsdzSWg5gDXhM58
QOswuA5j+I3aEdxn1mpAw0lzhaB9ct3Vf4Y6HuTnOewhdOwsRlzVDhigDrtPc2Zw
D8x32Oy7duUrB9QazxN0IIPkzeI70H9lMzhIjRo5MeilIhKK2KMzZQ==
-----END RSA PRIVATE KEY-----

Public Key : This is the public key in the original raw "X509" format. It's not usable in DNS directly, but it might be useful for something else.

-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAwkHYFBOunpjkTVI+706KnYAjTota7ws824czVIDJMpFX9JjaYou1
G++vN68A0YUV898xhgqMuIpBfm7pj+C0Q4biS7RCtAPCpx/i4+yYkB7gN/ikawok
BQif++7+Fsfh4Y+KTGQbvDw2T601j1fQrb87SPQMqsjDNmOmhHuOIRke9OPJ3jGD
LJVMYU/bky2VoPRAjkC7WMaVncDF54rARLOH3WTDIXjpWTWFa3dwKMFFY8WcuEVW
aQAHweKZFCGhkVO/t5tPr/QlDHFY6Zb+eymGIlafbtQDmlXmc9OCuxQgffu+0aAY
wgfoI0Wr6eA2ydjIaasVEtMjbz+abuT7xwIDAQAB
-----END RSA PUBLIC KEY-----
```

---
License: MIT