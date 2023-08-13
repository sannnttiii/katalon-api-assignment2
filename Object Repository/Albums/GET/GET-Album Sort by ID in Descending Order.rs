<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-Album Sort by ID in Descending Order</name>
   <tag></tag>
   <elementGuidId>5558c1a3-c58a-4b3a-87d0-dee33c1e9e01</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/albums?_sort=id&amp;_order=desc</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, '[0].userId', 10)
WS.verifyElementPropertyValue(response, '[0].id', 100)
WS.verifyElementPropertyValue(response, '[0].title', 'enim repellat iste')

WS.verifyElementPropertyValue(response, '[1].userId', 10)
WS.verifyElementPropertyValue(response, '[1].id', 99)
WS.verifyElementPropertyValue(response, '[1].title', 'consectetur ut id impedit dolores sit ad ex aut')

WS.verifyElementPropertyValue(response, '[2].userId', 10)
WS.verifyElementPropertyValue(response, '[2].id', 98)
WS.verifyElementPropertyValue(response, '[2].title', 'omnis quia possimus nesciunt deleniti assumenda sed autem')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
