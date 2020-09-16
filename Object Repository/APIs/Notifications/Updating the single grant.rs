<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For Updating the single grant</description>
   <name>Updating the single grant</name>
   <tag></tag>
   <elementGuidId>e68257e8-5ed8-4c3c-ba6f-b76b696b7c41</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/faculty/addGrantReceived?employeeID=5AB4B781DE61438B95F0D20F678088F5&amp;insertData=%7B%22department%22%3A%22201961010135731679305002%22%2C%22scheme%22%3A%22sdf%22%2C%22amtsan%22%3A%2250000%22%2C%22sanlettr%22%3A%22sdf441%22%2C%22utipostoday%22%3A%2245000%22%2C%22uticerdet%22%3A%22sdf552%22%2C%22yeargrant%22%3A%222015%22%7D&amp;state=forGrantRcvdInsert</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.URL</defaultValue>
      <description>url</description>
      <id>b6e3981e-5806-49ab-a9c2-1deec44893a5</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
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

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
