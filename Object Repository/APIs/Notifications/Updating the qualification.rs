<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For updating the qualification</description>
   <name>Updating the qualification</name>
   <tag></tag>
   <elementGuidId>d1dd2e80-54dc-42f9-8250-1ac52ab3943b</elementGuidId>
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
   <restUrl>${URL}/faculty/saveDynamicQualification?qualJsonData=%7B%22qualificationType%22%3A%22Post%20Graduation%22%2C%22clientID%22%3A%227BA54C5313174AE58FCB45B1A40F608A%22%2C%22updatedBy%22%3A%22100%22%2C%22setActive%22%3A%22Y%22%2C%22qualificationFileName%22%3A%22Qualification.jpg%22%2C%22qualificationID%22%3A%2222ABF15CD25811EAA4D8D36D2314F49C%22%2C%22createdBy%22%3A%22100%22%2C%22yearOfPass%22%3A%222019%22%2C%22branchName%22%3A%22MTECH%22%2C%22employeeID%22%3A%7B%22employeePrimaryId%22%3A%225AB4B781DE61438B95F0D20F678088F5%22%2C%22quaternaryprogram%22%3A%22undefined%22%2C%22surName%22%3A%22c%22%2C%22gender%22%3A%22Female%22%2C%22mobileNumber%22%3A%229000567484%22%2C%22aadharNo%22%3A%22123481224301%22%2C%22tertiaryprogram%22%3A%22undefined%22%2C%22firstName%22%3A%22C.%20Suguna%20Devi%22%2C%22secondaryprogram%22%3A%22undefined%22%2C%22quinaryprogram%22%3A%22undefined%22%2C%22password%22%3A%22DEMO051020%22%2C%22employeeType%22%3A%22Teaching%20Staff%22%2C%22alternateMobile%22%3A%227889955220%22%2C%22senaryprogram%22%3A%22undefined%22%2C%22designation%22%3A%22Assistant%20Professor%22%2C%22department%22%3A%7B%22departmentName%22%3A%2205%22%2C%22hodName%22%3A%22T.N.%20Ranganadham%22%2C%22hodID%22%3A%22466DDFD65934495A98466D0F5191E02C%22%2C%22departmentPrimaryId%22%3A%22201961010135731679305002%22%2C%22departmentdes%22%3A%22CSE%22%7D%2C%22email%22%3A%22sugana%40gmail.com%22%2C%22panCardNo%22%3A%22ABCDC5779L%22%2C%22username%22%3A%22DEMO051020%22%2C%22status%22%3A%22Y%22%2C%22septenaryprogram%22%3A%22undefined%22%7D%2C%22orgID%22%3A%220%22%2C%22%24%24hashKey%22%3A%22object%3A1265%22%2C%22collegeName%22%3A%22SVCEC%22%7D&amp;filename=256-2569968_nature-4k-wallpaper-for-laptops.jpg&amp;state=saveDynamicQualification</restUrl>
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
      <id>0f35da61-cb0b-43f9-85a8-da8b1e5b9f44</id>
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
