# NTLM Mediator

NTLM (Windows NT LAN Manager) is an authentication protocol provided in Windows server. NTLM authentication is based on a challenge response-based protocol and WSO2 API Manager gives support to access NTLM protected services by using the NTLM mediator. You need to configure the NTLM backend and use that credentials to access NTLM protected services by using the WSO2 API Manager. First you need to initialize the NTLM mediator and then you can use call mediator or callout mediator to send requests to the backend service.

---

## Info

- The NTLM mediator is a [content-unaware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

```java
<NTLM [username="string"] [password="string"] [host="string"] [domain="string"] [ntlmVersion="string"]>
</NTLM>
```
