<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Save Work experience</description>
   <name>Save Work experience</name>
   <tag></tag>
   <elementGuidId>12fb2d60-c1bd-4e83-83e6-ea56c0b7631a</elementGuidId>
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
   <restUrl>${URL}/faculty/saveWorkExperience?workJsonData=%7B%22workExperienceId%22%3A%22%22%2C%22employee%22%3A%7B%22employeePrimaryId%22%3A%225AB4B781DE61438B95F0D20F678088F5%22%7D%2C%22collegename%22%3A%22SACET%22%2C%22designation%22%3A%22Lab%20Co-ordinator%22%2C%22responsibilites%22%3A%22Non%20Teaching%20Staff%22%2C%22expFrom%22%3A%222019-01-01%22%2C%22expTo%22%3A%222019-05-31%22%7D&amp;filename=256-2569968_nature-4k-wallpaper-for-laptops.jpg&amp;state=saveWorkExperience</restUrl>
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
      <id>08234852-492e-4a65-aa9a-a8de97a505c2</id>
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
