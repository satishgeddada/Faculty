import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl(GlobalVariable.Web)

WebUI.setText(findTestObject('Web/Login/input_Faculty Login_username'), 'DEMO051020')

WebUI.setEncryptedText(findTestObject('Web/Login/input_Faculty Login_password'), '8wrFRVevT6nANR47r2EKIw==')

WebUI.click(findTestObject('Object Repository/Page_/button_Login'))

WebUI.click(findTestObject('Web/Profile/Personal Details/a_Profile'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Page_/button_Change Profile Information'))

WebUI.selectOptionByValue(findTestObject('Web/Profile/Personal Details/select_Choose your optionMrMrsMsMiss'), 'Ms.', true)

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_name'), 'C. Suguna Devi')

WebUI.click(findTestObject('Web/Profile/Personal Details/div_Update Profile'))

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Surname'), 'SUGUNA')

WebUI.selectOptionByValue(findTestObject('Web/Profile/Personal Details/select_select genderMaleFemaleOthers'), 'Female', 
    true)

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Date of Birth'), '03/05/1981')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Fathers Name'), 'SUBBARAO')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Employee Type'), 'Teaching Staff')

WebUI.selectOptionByValue(findTestObject('Web/Profile/Personal Details/select_select your optionRegularContractualVisiting'), 
    'Regular', true)

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Designation'), 'Assistant Professor')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Date of Joining'), '08/10/2006')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Aadhar Card No'), '1234567890')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_PAN Card No'), 'ABCDE1234F')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_JNTUA No'), '123456')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_AICTE No'), '123')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Achievements'), 'NO')

WebUI.setText(findTestObject('Web/Profile/Personal Details/input_Update Profile_Research Activities'), 'NO')

WebUI.click(findTestObject('Object Repository/Page_/button_Update'))

WebUI.takeScreenshot()

WebUI.closeBrowser()

