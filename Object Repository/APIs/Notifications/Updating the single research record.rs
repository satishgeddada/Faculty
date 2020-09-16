<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For updating  the single research record</description>
   <name>Updating the single research record</name>
   <tag></tag>
   <elementGuidId>db910d8c-2330-432d-800d-aa9fbe4c2c71</elementGuidId>
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
   <restUrl>${URL}/faculty/insertResearchGuidance?employeeID=5AB4B781DE61438B95F0D20F678088F5&amp;insertData=%7B%22scholarName%22%3A%22Srijan%22%2C%22yearOfAdmission%22%3A%222014%22%2C%22typeofAcademic%22%3A%22Ph.D%22%2C%22topic%22%3A%22dsfsdf%22%2C%22universityName%22%3A%22svu%22%2C%22status%22%3A%22Completed%22%2C%22ResearchGuidanceID%22%3A%222020731132958735%22%2C%22deptId%22%3A%22201961010135731679305002%22%7D&amp;state=forResearchGuidanceInsert</restUrl>
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
      <id>080cfa2c-4276-45f2-8f31-d45328ba622c</id>
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
