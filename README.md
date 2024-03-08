## webhook-provider

Today, many platforms offer integration through webhook, webhooks allow an application x to know about events on application y asynchronously, without the need of pooling a given endpoint. This is a study project, to learn how to build a webhook provider, like Github, Stripe or some other modern tool. 

### Github example

Let's say you want to send a message in a specific Discord channel everytime an issue is opened on your Github repository. To be able to listen to Github events, you need to access their "Webhook" page in your repository, select the events you want to subscribe to and the URL of your POST endpoint that Github needs to call when the events you want occur. 

- Github: the webhook provider
- My application: the one subscribed to Github events, it has a POST endpoint that process the payload and applies any logic needed to achieve my bussiness rule
- Payload: the message sent by Github, specifying the event that triggered the request

### This repo


In this repo, I'm aiming to build the a webhook provider just like Github, to simplify things, the events will be fake and they'll came from random data.

If you want to learn more about how webhook providers work, there's a special issue called read list, I'll bring together all the resources I found useful while studying about this topic. You call also help me writing issues and code, feel free to contribute!
