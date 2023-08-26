# Data Service Call Mediator

The **Data Service Call Mediator** is used to invoke data service operations. It automatically creates a payload and sets up the necessary headers to invoke the data service. Also, it improves the performance by directly calling the data service (without HTTP transport).

---

## Info

- You need to first have a [Data Service Project](https://apim.docs.wso2.com/en/latest/integrate/develop/creating-artifacts/data-services/creating-data-services/) to use the Data Service Call mediator.
- The Data Service Call mediator is a [content-aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators)  mediator.

## Syntax

``` java
<dataServiceCall serviceName="data-service-name">
   <source [type="inline" | "body"]/>
   <operations [type="single" | "batch" | "request-box"] >
      <operation name="operation-name">
         <param name="param-name" value="param-value"/>
      </operation>
   </operations>
   <target [type="body" | "property"] name="target-property-name"/>
</dataServiceCall>
```

