# Loopback Mediator

The **Loopback Mediator** moves messages from the in flow (request path) to the out flow (response path). All the configuration included in the in sequence that appears after the Loopback mediator is skipped.

---

## Info

- The Loopback mediator is a [content-unaware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.
- The messages that have already been passed from the In sequence to the Out sequence cannot be moved to the Out sequence again via the Loopback mediator. 

## Syntax

The loopback token refers to a `<loopback/>` element, which is used to skip the rest of the in flow and move the message to the out flow.

``` java
<loopback/>
```

