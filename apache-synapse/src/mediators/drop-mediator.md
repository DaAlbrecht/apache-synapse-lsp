# Drop Mediator

The **Drop Mediator** stops the processing of the current message. This mediator is useful for ensuring that the message is sent only once and
then dropped by the Micro Integrator. If you have any mediators defined after the `<drop/>` element, they will not be executed, because `<drop/>` is considered to be the end of the message flow.

When the Drop mediator is within the `In` sequence, it sends an HTTP 202 Accepted response to the client when it stops the message flow. When the Drop mediator is within the `Out` sequence before the Send mediator, no response is sent to the client.

---

## Info

The Drop mediator is a [content-unaware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

The drop token refers to a `<drop/>` element, which is used to stop further processing of a message:

``` java
<drop/>
```
