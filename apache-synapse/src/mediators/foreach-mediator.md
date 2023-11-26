# ForEach Mediator

The ForEach mediator requires an XPath/JSONPath expression and a sequence (inline or referred). It splits the message into a number of different messages
derived from the original message by finding matching elements for the
XPath/JSONPath expression specified. Based on the matching elements, new messages
are created for each iteration and processed sequentially. The
processing is carried out based on a specified sequence. The behaviour
of ForEach mediator is similar to a generic loop. After mediation, the
sub-messages are merged back to their original parent element in the
original message sequentially.

---

The ForEach mediator creates the following properties during mediation.

| Property                   | Description                                                                                           |
|----------------------------|-------------------------------------------------------------------------------------------------------|
| FOREACH_ORIGINAL_MESSAGE | This contains the original envelop of the messages split by the ForEach mediator.                     |
| FOREACH_COUNTER           | This contains the count of the messages processed. The message count increases during each iteration. |

## Note

[Iterate Mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/iterate-mediator/) is quite similar to the ForEach
mediator. You can use complex XPath expressions to conditionally select
elements to iterate over in both mediators. Following are the main
difference between ForEach and Iterate mediators:

-   Use the ForEach mediator only for message transformations. If you
    need to make back-end calls from each iteration, then use the
    iterate mediator.
-   ForEach supports modifying the original payload. You can use Iterate
    for situations where you send the split messages to a target and
    collect them by an Aggregate in a different flow
-   You need to always accompany an Iterate with an Aggregate mediator.
    ForEach loops over the sub-messages and merges them back to the same
    parent element of the message.
-   In Iterate you need to send the split messages to an endpoint to
    continue the message flow. However, ForEach does not allow using
    [Call](https://apim.docs.wso2.com/en/latest/reference/mediators/call-mediator/), [Send](https://apim.docs.wso2.com/en/latest/reference/mediators/send-mediator/) and
    [Callout](https://apim.docs.wso2.com/en/latest/reference/mediators/callout-mediator/) mediators in the sequence.
-   ForEach does not split the message flow, unlike Iterate Mediator. It
    guarantees to execute in the same thread until all iterations are
    complete.

When you use ForEach mediator, you can only loop through segments of the
message and do changes to a particular segment. For example, you can
change the payload using payload factory mediator. But you cannot send
the split message out to a service. Once you exit from the ForEach loop,
it automatically aggregates the split segments. This replaces the
ForEach function of the complex XSLT mediators using a ForEach mediator
and a Payload Factory mediator. However, to implement the
split-aggregate pattern, you still need to use Iterate mediator.

## Syntax

```java
<foreach expression="xpath|jsonpath" [sequence="sequence_ref"] [id="foreach_id"] >
    <sequence>
      (mediator)+
    </sequence>?
</foreach>
```
