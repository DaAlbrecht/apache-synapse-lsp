# FastXSLT Mediator

The **FastXSLT Mediator** is similar to the [XSLT mediator]({{base_path}}/reference/mediators/xslt-mediator), but it uses the [Streaming XPath Parser](https://wso2.com/library/articles/2013/01/streaming-xpath-parser-wso2-esb/) and applies the XSLT transformation to the message stream instead of to the XML message payload. The result is a faster transformation, but you cannot specify the source, properties, features, or resources as you can with the XSLT mediator. Therefore, the FastXSLT mediator is intended to be used to gain performance in cases where the original message remains unmodified. Any pre-processing performed on the message payload will not be visible to the FastXSLT mediator, because the transformation logic is applied on the original message stream instead of the message payload. In cases where the message payload needs to be pre-processed, use the XSLT mediator instead of the FastXSLT mediator.

---

## Note

The streaming XPath parser used in the Fast XSLT mediator does not support Xpath functions specified with the prefix `fn:`. Examples are  `fn:contains`, `fn:count`, and `fn:concat`.

## Syntax

``` java
<fastXSLT key="string"/>
```

For example, specify the XSLT by the key `transform/example.xslt`, which is used to transform the message stream as shown below.

``` java
<fastxslt xmlns="http://ws.apache.org/ns/synapse" key="transform/example.xslt"/>
```

