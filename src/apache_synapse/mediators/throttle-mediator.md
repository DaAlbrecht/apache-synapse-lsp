# Throttle Mediator

The **Throttle Mediator** can be used to restrict access to services.
This is useful when services used at the enterprise level and it is
required to avoid heavy loads that can cause performance issues in the
system. It can also be used when you want to avoid certain user groups
(i.e. IP addresses and domains) accessing your system. The Throttle
mediator defines a throttle group which includes the following.

-   A throttle policy which defines the extent to which, individuals and groups of IP addresses/domains should be allowed to access the
    service.
-   A mediation sequence to handle requests that were accepted based on the throttle policy.
-   A mediation sequence to handle requests that were rejected based on the throttle policy.

---

## Info

The Throttle mediator is a [content unaware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

``` java
<throttle [onReject="string"] [onAccept="string"] id="string">
    (<policy key="string"/> | <policy>..</policy>)
    <onReject>..</onReject>?
    <onAccept>..</onAccept>?
</throttle>
```

