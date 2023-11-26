# EJB Mediator

The **EJB mediator** calls an external Enterprise JavaBean(EJB) and stores the result in the message payload or in a message context property. Currently, this mediator supports EJB3 Stateless Session Beans and Stateful Session Beans.

---

## Info

The EJB mediator is a [content-aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

``` java
<ejb beanstalk="string" class="string" [sessionId="string"] [remove="true | false"] 
  [method="string"] [target="string | {xpath}"] [jndiName="string"] /> 
    <args> 
      <arg (value="string | {xpath}")/>* 
    </args> 
</ejb>
```
