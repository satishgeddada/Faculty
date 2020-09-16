<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For Save Qualification</description>
   <name>Save Qualification</name>
   <tag></tag>
   <elementGuidId>dd98f2d5-eac3-4eec-97a6-c31602ad6484</elementGuidId>
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
   <restUrl>${URL}/faculty/saveDynamicQualification?qualJsonData=%7B%22qualificationID%22%3A%22%22%2C%22employeeID%22%3A%7B%22employeePrimaryId%22%3A%225AB4B781DE61438B95F0D20F678088F5%22%7D%2C%22qualificationType%22%3A%22Graduation%2FUnderGraduation%22%2C%22collegeName%22%3A%22SVCE%22%2C%22branchName%22%3A%22BSC%22%2C%22yearOfPass%22%3A%222012%22%7D&amp;filename=256-2569968_nature-4k-wallpaper-for-laptops.jpg&amp;state=saveDynamicQualification</restUrl>
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
      <id>6c94e125-63ab-453d-a6f3-92ef39a6ba59</id>
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
