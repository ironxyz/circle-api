# \SubscriptionsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscription**](SubscriptionsApi.md#create_subscription) | **POST** /v1/notifications/subscriptions | Create a notification subscription
[**delete_subscription**](SubscriptionsApi.md#delete_subscription) | **DELETE** /v1/notifications/subscriptions/{id} | Remove a notification subscription
[**list_subscriptions**](SubscriptionsApi.md#list_subscriptions) | **GET** /v1/notifications/subscriptions | List all notification subscriptions



## create_subscription

> models::CreateSubscriptionResponse create_subscription(subscription_request)
Create a notification subscription

Subscribe to receiving notifications at a given endpoint. The endpoint should be able to handle AWS SNS subscription requests. For more details see https://docs.aws.amazon.com/mobile/sdkforxamarin/developerguide/sns-send-http.html. Note, the sandbox environment allows a maximum of 3 active subscriptions; otherwise, this is limited to 1 active subscription and subsequent create requests will be rejected with a Limit Exceeded error. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_request** | Option<[**SubscriptionRequest**](SubscriptionRequest.md)> |  |  |

### Return type

[**models::CreateSubscriptionResponse**](CreateSubscriptionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription

> models::DeleteSubscriptionResponse delete_subscription(id)
Remove a notification subscription

To remove a subscription, all its subscription requests' statuses must be either 'confirmed', 'deleted' or a combination of those. A subscription with at least one 'pending' subscription request cannot be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::DeleteSubscriptionResponse**](DeleteSubscriptionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subscriptions

> models::ListSubscriptionsResponse list_subscriptions()
List all notification subscriptions

Retrieve a list of existing notification subscriptions with details.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListSubscriptionsResponse**](ListSubscriptionsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

