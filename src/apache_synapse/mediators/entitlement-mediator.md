# Entitlement Mediator

The **Entitlement Mediator** intercepts requests and evaluates the actions performed by a user against an [eXtensible Access Control Markup Language (XACML)](http://en.wikipedia.org/wiki/XACML) policy. This supports XACML 2.0 and 3.0. WSO2 Identity Server can be used as the XACML Policy Decision Point (PDP) where the policy is set, and the Micro Integrator serves as the XACML Policy Enforcement Point (PEP) where the policy is enforced.

---

## Syntax

``` java
<entitlementService remoteServiceUrl="" remoteServiceUserName="" remoteServicePassword="" callbackClass="org.wso2.carbon.identity.entitlement.mediator.callback.[UTEntitlementCallbackHandler|X509EntitlementCallbackHandler|SAMLEntitlementCallbackHandler|KerberosEntitlementCallbackHandler]" 
client="soap|basicAuth|thrift|wsXacml">
   <onReject/>
   <onAccept/>
   <advice/>
   <obligations/>
</entitlementService>
```
