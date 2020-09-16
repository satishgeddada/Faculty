<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For saving the single Research record</description>
   <name>Saving the single Research record</name>
   <tag></tag>
   <elementGuidId>d8708854-4f1d-45dc-8cdc-7aa2dcc77787</elementGuidId>
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
   <restUrl>${URL}/faculty/insertResearchGuidance?employeeID=5AB4B781DE61438B95F0D20F678088F5&amp;insertData=%7B%22scholarName%22%3A%22Srijan%22%2C%22yearOfAdmission%22%3A%222014%22%2C%22typeofAcademic%22%3A%22Ph.D%22%2C%22topic%22%3A%22dsfsdf%22%2C%22universityName%22%3A%22svu%22%2C%22status%22%3A%22Submitted%22%2C%22deptId%22%3A%22201961010135731679305002%22%7D&amp;state=forResearchGuidanceInsert</restUrl>
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
      <id>8b74a49f-9263-40af-a1f6-6b265f5ac517</id>
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
