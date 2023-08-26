# Cache Mediator

When a message enters a message flow, the Cache mediator checks whether the incoming message is similar to a previous message that was received
within a specified period of time. This is done by evaluating the hash value of incoming messages. If a similar message was identified before, the Cache mediator executes the `onCacheHit` sequence (if specified), fetches the cached response, and prepares the Micro Integrator to send the response. The `onCacheHit` sequence can send back the response message using the [Respond Mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/respond-mediator/). If the `onCacheHit` sequence is not specified, the cached response is sent back to the requester and the message is not passed on. If a similar message has not been seen before, then the message is passed on.

---

##  Info

- The Cache mediator is a [content-aware]({{base_path}}/reference/mediators/about-mediators/#classification-of-mediators) mediator.
- The Cache mediator supports only local caching. It does not support distributed caching.

## Syntax

``` java
<cache [timeout="seconds"] [collector=(true | false)] [maxMessageSize="in-bytes"] >
   <onCacheHit [sequence="key"]>
    (mediator)+
   </onCacheHit>?
   <protocol type="http" >?
     <methods>comma separated list</methods>
     <headersToExcludeInHash>comma separated list</headersToExcludeInHash>
     <responseCodes>regular expression</responseCodes>
     <enableCacheControl>(true | false)</enableCacheControl>
     <includeAgeHeader>(true | false)</includeAgeHeader>
     <hashGenerator>class</hashGenerator>
   </protocol>    
   <implementation [maxSize="int"]/>
</cache>
```
