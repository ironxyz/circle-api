# SimpleCard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**status** | [**models::ExternalFiatAccountStatus**](ExternalFiatAccountStatus.md) |  | 
**billing_details** | [**models::SimpleBillingDetails**](SimpleBillingDetails.md) |  | 
**exp_month** | **i32** | Two digit number representing the card's expiration month. | 
**exp_year** | **i32** | Four digit number representing the card's expiration year. | 
**network** | **String** | The network of the card. | 
**bin** | Option<**String**> | The bank identification number (BIN), the first 6 digits of the card. | [optional]
**issuer_country** | Option<**String**> | The country code of the issuer bank. Follows the ISO 3166-1 alpha-2 standard. | [optional]
**funding_type** | Option<**String**> | The funding type of the card. Possible values are `credit`, `debit`, `prepaid`, and `unknown`. | [optional]
**fingerprint** | **String** | A UUID that uniquely identifies the account number. If the same account is used more than once, each card object will have a different id, but the fingerprint will stay the same. | 
**verification** | [**models::CardVerificationResponse**](CardVerificationResponse.md) |  | 
**risk_evaluation** | Option<[**models::RiskEvaluation**](RiskEvaluation.md)> |  | [optional]
**create_date** | **String** | ISO-8601 UTC date/time format. | 
**update_date** | **String** | ISO-8601 UTC date/time format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


