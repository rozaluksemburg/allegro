openapi: 3.1.0
info:
title: Mistral AI API
description: >-
Our Chat Completion and Embeddings APIs specification. Create your account
on [La Plateforme](https://console.mistral.ai) to get access and read the
[docs](https://docs.mistral.ai) to learn how to use it.
version: 0.0.2
servers:
- url: https://api.mistral.ai/v1
  paths:
  /chat/completions:
  post:
  operationId: createChatCompletion
  summary: Create Chat Completions
  requestBody:
  required: true
  content:
  application/json:
  schema:
  anyOf:
  - $ref: '#/components/schemas/ChatCompletionRequest'
  - $ref: '#/components/schemas/ChatCompletionRequestFunctionCall'
  - $ref: '#/components/schemas/ChatCompletionRequestJSONMode'
  responses:
  '200':
  description: OK
  content:
  application/json:
  schema:
  oneOf:
  - $ref: '#/components/schemas/ChatCompletionResponse'
  - $ref: '#/components/schemas/ChatCompletionResponseFunctionCall'
  - $ref: '#/components/schemas/ChatCompletionResponseJSONMode'
  /fim/completions:
  post:
  operationId: createFIMCompletion
  summary: Create FIM Completions
  requestBody:
  required: true
  content:
  application/json:
  schema:
  $ref: '#/components/schemas/FIMCompletionRequest'
  responses:
  '200':
  description: OK
  content:
  application/json:
  schema:
  $ref: '#/components/schemas/FIMCompletionResponse'
  /embeddings:
  post:
  operationId: createEmbedding
  summary: Create Embeddings
  requestBody:
  required: true
  content:
  application/json:
  schema:
  $ref: '#/components/schemas/EmbeddingRequest'
  responses:
  '200':
  description: OK
  content:
  application/json:
  schema:
  $ref: '#/components/schemas/EmbeddingResponse'
  /models:
  get:
  operationId: listModels
  summary: List Available Models
  responses:
  '200':
  description: OK
  content:
  application/json:
  schema:
  $ref: '#/components/schemas/ModelList'
  delete:
  summary: Delete Model
  description: Delete a fine-tuned model.
  operationId: delete_model_v1_models__model_id__delete
  parameters:
  - name: model_id
  in: path
  required: true
  schema:
  type: string
  title: Model Id
  responses:
  '200':
  description: Successful Response
  content:
  application/json:
  schema:
  $ref: '#/components/schemas/DeleteModelOut'
  '422':
  description: Validation Error
  content:
  application/json:
  schema:
  $ref: '#/components/schemas/HTTPValidationError'
  /files:
  post:
  operationId: files_api_routes_upload_file
  summary: Upload File
  parameters: []
  responses:
  '200':
  description: OK
  content:
  application/json:
  schema:
  $ref: '#/components/schemas/UploadFileOut'
  description: >-
  Upload a file that can be used across various endpoints.


        The size of individual files can be a maximum of 512 MB. The Fine-tuning
        API only supports .jsonl files.


        Please contact us if you need to increase these storage limits.
      requestBody:
        content:
          multipart/form-data:
            schema:
              title: MultiPartBodyParams
              type: object
              properties:
                purpose:
                  const: fine-tune
                  title: Purpose
                  description: >-
                    The intended purpose of the uploaded file. Only accepts
                    fine-tuning (`fine-tune`) for now.
                  example: fine-tune
                file:
                  format: binary
                  title: File
                  type: string
                  description: >
                    The File object (not file name) to be uploaded. 


                    To upload a file and specify a custom file name you should
                    format your request as such:
                      ```
                      file=@path/to/your/file.jsonl;filename=custom_name.jsonl
                      ```

                    Otherwise, you can just keep the original file name:
                      ```
                      file=@path/to/your/file.jsonl
                      ```
              required:
                - purpose
                - file
        required: true
    get:
      operationId: files_api_routes_list_files
      summary: List Files
      parameters: []
      responses:
        '200':
          description: OK
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ListFilesOut'
      description: Returns a list of files that belong to the user's organization.
/files/{file_id}:
get:
operationId: files_api_routes_retrieve_file
summary: Retrieve File
parameters:
- in: path
name: file_id
schema:
title: File Id
type: string
description: The ID of the file to use for this request.
required: true
responses:
'200':
description: OK
content:
application/json:
schema:
$ref: '#/components/schemas/RetrieveFileOut'
description: Returns information about a specific file.
delete:
operationId: files_api_routes_delete_file
summary: Delete File
parameters:
- in: path
name: file_id
schema:
title: File Id
type: string
description: The ID of the file to use for this request.
required: true
responses:
'200':
description: OK
content:
application/json:
schema:
$ref: '#/components/schemas/DeleteFileOut'
description: Delete a file.
/fine_tuning/jobs:
get:
operationId: jobs_api_routes_fine_tuning_get_fine_tuning_jobs
summary: List Fine Tuning Jobs
parameters:
- in: query
name: page
schema:
default: 0
title: Page
type: integer
required: false
description: The page number of the results to be returned.
- in: query
name: page_size
schema:
default: 100
title: Page Size
type: integer
required: false
description: The number of items to return per page.
- in: query
name: model
schema:
type: string
title: Model
required: false
description: >-
The model name used for fine-tuning to filter on. When set, the
other results are not displayed.
- in: query
name: status
schema:
type: string
enum:
- QUEUED
- STARTED
- RUNNING
- FAILED
- SUCCESS
- CANCELLED
- CANCELLATION_REQUESTED
title: Status
required: false
description: >-
The current job state to filter on. When set, the other results are
not displayed.
- in: query
name: created_after
schema:
type: string
format: datetime
nullable: true
description: >-
The date/time to filter on. When set, the results for previous
creation times are not displayed.
required: false
- in: query
name: created_by_me
schema:
type: bool
default: false
description: >-
When set, only return results for jobs created by the API caller.
Other results are not displayed.
- in: query
name: wandb_project
schema:
type: string
nullable: true
description: >-
The Weights and Biases project to filter on. When set, the other
results are not displayed.
- in: query
name: wandb_name
schema:
type: string
nullable: true
description: >-
The Weight and Biases run name to filter on. When set, the other
results are not displayed.
- in: query
name: suffix
schema:
type: string
nullable: true
description: >-
The model suffix to filter on. When set, the other results are not
displayed.
responses:
'200':
description: OK
content:
application/json:
schema:
$ref: '#/components/schemas/JobsOut'
description: Get a list of fine tuning jobs for your organization and user.
post:
operationId: jobs_api_routes_fine_tuning_create_fine_tuning_job
summary: Create Fine Tuning Job
parameters:
- in: query
name: dry_run
schema:
type: bool
default: false
description: >
* If `true` the job is not spawned, instead the query returns a
handful of useful metadata
for the user to perform sanity checks (see `JobMetadata` response).
* Otherwise, the job is started and the query returns the job ID
along with some of the
input parameters (see `JobOut` response).
responses:
'200':
description: OK
content:
application/json:
schema:
oneOf:
- $ref: '#/components/schemas/JobOut'
- $ref: '#/components/schemas/JobMetadata'
description: Create a new fine tuning job, it will be queued for processing.
requestBody:
content:
application/json:
schema:
$ref: '#/components/schemas/JobIn'
required: true
/fine_tuning/jobs/{job_id}:
get:
operationId: jobs_api_routes_fine_tuning_get_fine_tuning_job
summary: Get Fine Tuning Job
parameters:
- in: path
name: job_id
schema:
format: uuid
title: Job Id
type: string
description: The ID of the job to analyse.
required: true
responses:
'200':
description: OK
content:
application/json:
schema:
$ref: '#/components/schemas/DetailedJobOut'
description: Get a fine tuned job details by its UUID.
/fine_tuning/jobs/{job_id}/cancel:
post:
operationId: jobs_api_routes_fine_tuning_cancel_fine_tuning_job
summary: Cancel Fine Tuning Job
parameters:
- in: path
name: job_id
schema:
format: uuid
title: Job Id
type: string
required: true
responses:
'200':
description: OK
content:
application/json:
schema:
$ref: '#/components/schemas/DetailedJobOut'
description: Request the cancellation of a fine tuning job.
security:
- ApiKeyAuth: []
  components:
  securitySchemes:
  ApiKeyAuth:
  type: http
  scheme: bearer
  schemas:
  Error:
  type: object
  properties:
  type:
  type: string
  nullable: false
  message:
  type: string
  nullable: false
  param:
  type: string
  nullable: true
  code:
  type: string
  nullable: true
  required:
  - type
  - message
  - param
  - code
  ErrorResponse:
  type: object
  properties:
  error:
  $ref: '#/components/schemas/Error'
  required:
  - error
  ModelList:
  type: object
  properties:
  object:
  type: string
  data:
  type: array
  items:
  $ref: '#/components/schemas/Model'
  required:
  - object
  - data
  ChatCompletionRequest:
  type: object
  title: Regular
  properties:
  model:
  description: >
  ID of the model to use. You can use the [List Available
  Models](/api#operation/listModels) API to see all of your available
  models, or see our [Model overview](/models) for model descriptions.
  type: string
  example: mistral-small-latest
  messages:
  description: >
  The prompt(s) to generate completions for, encoded as a list of dict
  with role and content.
  type: array
  items:
  type: object
  properties:
  role:
  type: string
  enum:
  - system
  - user
  - assistant
  - tool
  content:
  type: string
  prefix:
  type: bool
  description: >
  **Only for the `assistant` role**


                  Set this to `true` when adding an assistant message as prefix
                  to condition the model response.

                  The role of the prefix message is to force the model to start
                  its answer by the content of

                  the message.
            example:
              role: user
              content: Who is the best French painter? Answer in one short sentence.
        temperature:
          type: number
          minimum: 0
          maximum: 1
          default: 0.7
          nullable: true
          description: >
            What sampling temperature to use, between 0.0 and 1.0. Higher values
            like 0.8 will make the output more random, while lower values like
            0.2 will make it more focused and deterministic.


            We generally recommend altering this or `top_p` but not both.
        top_p:
          type: number
          minimum: 0
          maximum: 1
          default: 1
          nullable: true
          description: >
            Nucleus sampling, where the model considers the results of the
            tokens with `top_p` probability mass. So 0.1 means only the tokens
            comprising the top 10% probability mass are considered.


            We generally recommend altering this or `temperature` but not both.
        max_tokens:
          type: integer
          minimum: 0
          default: null
          nullable: true
          example: 512
          description: >
            The maximum number of tokens to generate in the completion.


            The token count of your prompt plus `max_tokens` cannot exceed the
            model's context length. 
        stream:
          type: boolean
          default: false
          nullable: true
          description: >
            Whether to stream back partial progress. If set, tokens will be sent
            as data-only server-sent events as they become available, with the
            stream terminated by a data: [DONE] message. Otherwise, the server
            will hold the request open until the timeout or until completion,
            with the response containing the full result as JSON.
        safe_prompt:
          type: boolean
          default: false
          description: |
            Whether to inject a safety prompt before all conversations.
        random_seed:
          type: integer
          default: null
          example: 1337
          description: >
            The seed to use for random sampling. If set, different calls will
            generate deterministic results.
      required:
        - model
        - messages
    ChatCompletionRequestJSONMode:
      type: object
      title: JSON mode
      properties:
        model:
          description: >
            ID of the model to use. You can use the [List Available
            Models](/api#operation/listModels) API to see all of your available
            models, or see our [Model overview](/models) for model descriptions.
          type: string
          example: mistral-small-latest
        messages:
          description: >
            The prompt(s) to generate completions for, encoded as a list of dict
            with role and content. The first prompt role should be `user` or
            `system`.
          type: array
          items:
            type: object
            properties:
              role:
                type: string
                enum:
                  - system
                  - user
                  - assistant
                  - tool
              content:
                type: string
            example:
              role: user
              content: Who is the best French painter? Answer in JSON.
        response_format:
          type: object
          description: >
            An object specifying the format that the model must output. Setting
            to `{ "type": "json_object" }` enables JSON mode, which guarantees
            the message the model generates is in JSON.

            When using JSON mode you MUST also instruct the model to produce
            JSON yourself with a system or a user message.
          properties:
            type:
              type: string
              example: json_object
        temperature:
          type: number
          minimum: 0
          maximum: 1
          default: 0.7
          nullable: true
          description: >
            What sampling temperature to use, between 0.0 and 1.0. Higher values
            like 0.8 will make the output more random, while lower values like
            0.2 will make it more focused and deterministic.


            We generally recommend altering this or `top_p` but not both.
        top_p:
          type: number
          minimum: 0
          maximum: 1
          default: 1
          nullable: true
          description: >
            Nucleus sampling, where the model considers the results of the
            tokens with `top_p` probability mass. So 0.1 means only the tokens
            comprising the top 10% probability mass are considered.


            We generally recommend altering this or `temperature` but not both.
        max_tokens:
          type: integer
          minimum: 0
          default: null
          nullable: true
          example: 512
          description: >
            The maximum number of tokens to generate in the completion.


            The token count of your prompt plus `max_tokens` cannot exceed the
            model's context length. 
        stream:
          type: boolean
          default: false
          nullable: true
          description: >
            Whether to stream back partial progress. If set, tokens will be sent
            as data-only server-sent events as they become available, with the
            stream terminated by a data: [DONE] message. Otherwise, the server
            will hold the request open until the timeout or until completion,
            with the response containing the full result as JSON.
        safe_prompt:
          type: boolean
          default: false
          description: |
            Whether to inject a safety prompt before all conversations.
        random_seed:
          type: integer
          default: null
          example: 1337
          description: >
            The seed to use for random sampling. If set, different calls will
            generate deterministic results.
      required:
        - model
        - messages
    ChatCompletionRequestFunctionCall:
      type: object
      title: Function calling
      properties:
        model:
          description: >
            ID of the model to use. You can use the [List Available
            Models](/api#operation/listModels) API to see all of your available
            models, or see our [Model overview](/models) for model descriptions.
          type: string
          example: mistral-small-latest
        messages:
          description: >
            The prompt(s) to generate completions for, encoded as a list of dict
            with role and content. The first prompt role should be `user` or
            `system`.

            When role is `tool`, the properties should contain `tool_call_id`
            (string or `null`).
          type: array
          items:
            type: object
            properties:
              role:
                type: string
                enum:
                  - system
                  - user
                  - assistant
                  - tool
              content:
                type: string
            example:
              role: user
              content: What is the weather like in Paris?
        temperature:
          type: number
          minimum: 0
          maximum: 1
          default: 0.7
          nullable: true
          description: >
            What sampling temperature to use, between 0.0 and 1.0. Higher values
            like 0.8 will make the output more random, while lower values like
            0.2 will make it more focused and deterministic.


            We generally recommend altering this or `top_p` but not both.
        top_p:
          type: number
          minimum: 0
          maximum: 1
          default: 1
          nullable: true
          description: >
            Nucleus sampling, where the model considers the results of the
            tokens with `top_p` probability mass. So 0.1 means only the tokens
            comprising the top 10% probability mass are considered.


            We generally recommend altering this or `temperature` but not both.
        max_tokens:
          type: integer
          minimum: 0
          default: null
          example: 64
          nullable: true
          description: >
            The maximum number of tokens to generate in the completion.


            The token count of your prompt plus `max_tokens` cannot exceed the
            model's context length. 
        stream:
          type: boolean
          default: false
          nullable: true
          description: >
            Whether to stream back partial progress. If set, tokens will be sent
            as data-only server-sent events as they become available, with the
            stream terminated by a data: [DONE] message. Otherwise, the server
            will hold the request open until the timeout or until completion,
            with the response containing the full result as JSON.
        safe_prompt:
          type: boolean
          default: false
          description: |
            Whether to inject a safety prompt before all conversations.
        tools:
          type: array
          description: >
            A list of available tools for the model. Use this to specify
            functions for which the model can generate JSON inputs.
          items:
            type: object
            required:
              - type
              - function
            properties:
              type:
                type: string
                description: |
                  The type of the tool. Currently, only `function` is supported.
                example: function
              function:
                type: object
                required:
                  - name
                description: |
                  The function properties.
                properties:
                  description:
                    type: string
                    description: >
                      The description of the function to help the model
                      determine when and how to invoke it.
                    example: Get the current weather in a given location.
                  name:
                    type: string
                    required: true
                    description: >
                      The name of the function to be called. Must be a-z,A-Z,0-9
                      or contain underscores and dashes, with a maximum length
                      of 64.
                    example: get_weather
                  parameters:
                    type: object
                    description: >
                      The function parameters, defined using a JSON Schema
                      object. If omitted, the function is considered to have an
                      empty parameter list.
                    example:
                      type: object
                      properties:
                        location:
                          type: string
                          description: The city and department, e.g. Marseille, 13
                        unit:
                          type: string
                          enum:
                            - celsius
                            - fahrenheit
                      required:
                        - location
        tool_choice:
          type: string
          default: auto
          description: >
            Specifies if/how functions are called. If set to `none` the model
            won't call a function and will generate a message instead. If set to
            `auto` the model can choose to either generate a message or call a
            function. If set to `any` the model is forced to call a function.
          example: auto
        random_seed:
          type: integer
          default: null
          example: 1337
          description: >
            The seed to use for random sampling. If set, different calls will
            generate deterministic results.
      required:
        - model
        - messages
    FIMCompletionRequest:
      properties:
        prompt:
          type: string
          description: The text/code to complete.
          example: def
        suffix:
          type: string
          nullable: true
          description: |
            Optional text/code that adds more context for the model.
            When given a `prompt` and a `suffix` the model will fill
            what is between them. When `suffix` is not provided, the
            model will simply execute completion starting with
            `prompt`.
          example: return a+b
        model:
          type: string
          nullable: true
          description: |
            ID of the model to use. Only compatible for now with:
              - `codestral-2405`
              - `codestral-latest` 
          example: codestral-latest
        temperature:
          type: number
          maximum: 1
          minimum: 0
          default: 0.7
          nullable: true
          description: |
            What sampling temperature to use, between 0.0 and 1.0. 
            Higher values like 0.8 will make the outptu more random,
            while lower values like 0.2 will make it more focused and
            deterministic.

            We generally recommend altering this or `top_p` but not both.
          example: 0
        top_p:
          type: number
          maximum: 1
          minimum: 0
          default: 1
          nullable: true
          description: |
            Nucleus sampling, where the model considers the results of the
            tokens with with `top_p` probability mass. So 0.1 means only
            the tokens comprising the top 10% probability mass are considered.

            We generally recommend altering this or `temperature` but not both.
          example: 1
        max_tokens:
          type: integer
          minimum: 0
          nullable: true
          description: |
            The maximum number of tokens to generate in the completion.

            The token count of your prompt plus `max_tokens` cannot
            exceed the model's context length.
          example: 1024
        min_tokens:
          type: integer
          minimum: 0
          nullable: true
          description: |
            The minimum number of tokens to generate in the completion.
        stream:
          type: boolean
          default: false
          description: |
            Whether to stream back partial progress. If set, tokens will be
            sent as data-only server-side events as they become available,
            with the stream terminated by a data: [DONE] message." 
            Otherwise, the server will hold the request open until the timeout
            or until completion, with the response containing the full result
            as JSON.
          example: false
        random_seed:
          type: integer
          minimum: 0
          nullable: true
          description: |
            The seed to use for random sampling. If set, different calls will
            generate deterministic results.
          example: 1337
        stop:
          anyOf:
            - type: string
              description: Stop generation if this token is detected.
            - type: array
              items:
                type: string
                description: Stop generation if one of these tokens is detected.
          default: []
      type: object
      required:
        - prompt
        - model
    ChatCompletionResponse:
      type: object
      title: Regular
      properties:
        id:
          type: string
          example: cmpl-e5cc70bb28c444948073e77776eb30ef
        object:
          type: string
          example: chat.completion
        created:
          type: integer
          example: 1702256327
        model:
          type: string
          example: mistral-small-latest
        choices:
          type: array
          items:
            type: object
            required:
              - index
              - text
              - finish_reason
            properties:
              index:
                type: integer
                example: 0
              message:
                type: object
                properties:
                  role:
                    type: string
                    enum:
                      - user
                      - assistant
                    example: assistant
                  content:
                    type: string
                    example: >-
                      Claude Monet is often considered one of the best French
                      painters due to his significant role in the Impressionist
                      movement.
              finish_reason:
                type: string
                enum:
                  - stop
                  - length
                  - model_length
                  - error
                  - tool_calls
                example: stop
        usage:
          type: object
          properties:
            prompt_tokens:
              type: integer
              example: 16
            completion_tokens:
              type: integer
              example: 34
            total_tokens:
              type: integer
              example: 50
          required:
            - prompt_tokens
            - completion_tokens
            - total_tokens
    ChatCompletionResponseJSONMode:
      type: object
      title: JSON mode
      properties:
        id:
          type: string
          example: cmpl-e5cc70bb28c444948073e77776eb30ef
        object:
          type: string
          example: chat.completion
        created:
          type: integer
          example: 1702256327
        model:
          type: string
          example: mistral-small-latest
        choices:
          type: array
          items:
            type: object
            required:
              - index
              - text
              - finish_reason
            properties:
              index:
                type: integer
                example: 0
              message:
                type: object
                properties:
                  role:
                    type: string
                    enum:
                      - user
                      - assistant
                    example: assistant
                  content:
                    type: string
                    example: >-
                      {"name": "Claude Monet", "reason": "Claude Monet is often
                      considered one of the best French painters due to his
                      significant role in the development of Impressionism, a
                      major art movement that originated in France. His water
                      lily paintings are among the most famous works in the
                      history of art."}
              finish_reason:
                type: string
                enum:
                  - stop
                  - length
                  - model_length
                  - error
                  - tool_calls
                example: stop
        usage:
          type: object
          properties:
            prompt_tokens:
              type: integer
              example: 14
            completion_tokens:
              type: integer
              example: 83
            total_tokens:
              type: integer
              example: 69
          required:
            - prompt_tokens
            - completion_tokens
            - total_tokens
    ChatCompletionResponseFunctionCall:
      type: object
      title: Function calling
      properties:
        id:
          type: string
          example: cmpl-e5cc70bb28c444948073e77776eb30ef
        object:
          type: string
          example: chat.completion
        created:
          type: integer
          example: 1702256327
        model:
          type: string
          example: mistral-large-latest
        choices:
          type: array
          items:
            type: object
            required:
              - index
              - text
              - finish_reason
            properties:
              index:
                type: integer
                example: 0
              message:
                type: object
                properties:
                  role:
                    type: string
                    example: assistant
                  content:
                    type: string
                    example: ''
                  tool_calls:
                    type: array
                    items:
                      type: object
                      properties:
                        function:
                          type: object
                          properties:
                            name:
                              type: string
                            arguments:
                              type: str
                    example:
                      - function:
                          name: get_current_weather
                          arguments: '{"location": "Paris, 75"}'
              finish_reason:
                type: string
                enum:
                  - stop
                  - length
                  - model_length
                  - error
                  - tool_calls
                example: tool_calls
        usage:
          type: object
          properties:
            prompt_tokens:
              type: integer
              example: 118
            completion_tokens:
              type: integer
              example: 35
            total_tokens:
              type: integer
              example: 153
          required:
            - prompt_tokens
            - completion_tokens
            - total_tokens
    EmbeddingRequest:
      type: object
      properties:
        model:
          type: string
          example: mistral-embed
          description: |
            The ID of the model to use for this request.
        input:
          type: array
          items:
            type: string
          example:
            - Hello
            - world
          description: |
            The list of strings to embed.
        encoding_format:
          type: string
          enum:
            - float
          example: float
          description: |
            The format of the output data.
    EmbeddingResponse:
      type: object
      properties:
        id:
          type: string
          example: embd-aad6fc62b17349b192ef09225058bc45
        object:
          type: string
          example: list
        data:
          type: array
          items:
            type: object
            properties:
              object:
                type: string
                example: embedding
              embedding:
                type: array
                items:
                  type: number
                example:
                  - 0.1
                  - 0.2
                  - 0.3
              index:
                type: int
                example: 0
          example:
            - object: embedding
              embedding:
                - 0.1
                - 0.2
                - 0.3
              index: 0
            - object: embedding
              embedding:
                - 0.4
                - 0.5
                - 0.6
              index: 1
        model:
          type: string
        usage:
          type: object
          properties:
            prompt_tokens:
              type: integer
              example: 9
            total_tokens:
              type: integer
              example: 9
          required:
            - prompt_tokens
            - total_tokens
      required:
        - id
        - object
        - data
        - model
        - usage
    Model:
      title: Model
      description: Model object.
      properties:
        id:
          type: string
        object:
          type: string
        created:
          type: integer
        owned_by:
          type: string
      required:
        - id
        - object
        - created
        - owned_by
    UploadFileOut:
      properties:
        id:
          format: uuid
          title: Id
          type: string
          description: The ID of the created file.
        object:
          title: Object
          type: string
          example: file
        bytes:
          title: Bytes
          type: integer
          description: The size (in bytes) of the created file.
          example: 12000
        created_at:
          title: Created At
          type: integer
          description: The UNIX timestamp (in seconds) for the creation time of the file.
          example: 1717491627
        filename:
          title: Filename
          type: string
          description: The name of the file that was uploaded.
          example: train.jsonl
        purpose:
          const: fine-tune
          title: Purpose
      required:
        - id
        - object
        - bytes
        - created_at
        - filename
        - purpose
      title: UploadFileOut
      type: object
    ListFilesOut:
      properties:
        data:
          items:
            $ref: '#/components/schemas/FileSchema'
          title: Data
          type: array
        object:
          title: Object
          type: string
      required:
        - data
        - object
      title: ListFilesOut
      type: object
    RetrieveFileOut:
      properties:
        id:
          format: uuid
          title: Id
          type: string
        object:
          title: Object
          type: string
        bytes:
          title: Bytes
          type: integer
        created_at:
          title: Created At
          type: integer
        filename:
          title: Filename
          type: string
        purpose:
          const: fine-tune
          title: Purpose
      required:
        - id
        - object
        - bytes
        - created_at
        - filename
        - purpose
      title: RetrieveFileOut
      type: object
    DeleteFileOut:
      properties:
        id:
          format: uuid
          title: Id
          type: string
          description: The ID of the deleted file.
          example: 97f6eca-6276-4993-bfeb-53cbbbba6f08
        object:
          title: Object
          type: string
          description: The object type that was deleted
          default: file
        deleted:
          title: Deleted
          type: boolean
          description: The deletion status.
      required:
        - id
        - object
        - deleted
      title: DeleteFileOut
      type: object
    DeleteModelOut:
      properties:
        id:
          format: uuid
          title: Id
          type: string
          description: The ID of the deleted model.
          example: ft:open-mistral-7b:587a6b29:20240514:7e773925
        object:
          title: Object
          type: string
          description: The object type that was deleted
          default: model
        deleted:
          title: Deleted
          type: boolean
          description: The deletion status
          example: true
      required:
        - id
        - object
        - deleted
      title: DeleteModelOut
      type: object
    FineTuneableModel:
      enum:
        - open-mistral-7b
        - mistral-small-latest
      title: FineTuneableModel
      type: string
      description: The name of the model to fine-tune.
    JobOut:
      properties:
        id:
          format: uuid
          title: Id
          type: string
          description: The ID of the job.
        hyperparameters:
          $ref: '#/components/schemas/TrainingParameters'
        model:
          $ref: '#/components/schemas/FineTuneableModel'
        status:
          enum:
            - QUEUED
            - STARTED
            - RUNNING
            - FAILED
            - SUCCESS
            - CANCELLED
            - CANCELLATION_REQUESTED
          title: Status
          type: string
          description: The current status of the fine-tuning job.
        job_type:
          title: Job Type
          type: string
          description: The type of job (`FT` for fine-tuning).
        created_at:
          title: Created At
          type: integer
          description: >-
            The UNIX timestamp (in seconds) for when the fine-tuning job was
            created.
        modified_at:
          title: Modified At
          type: integer
          description: >-
            The UNIX timestamp (in seconds) for when the fine-tuning job was
            last modified.
        training_files:
          items:
            format: uuid
            type: string
          title: Training Files
          type: array
          description: >-
            A list containing the IDs of uploaded files that contain training
            data.
        validation_files:
          items:
            format: uuid
            type: string
          type: array
          default: []
          title: Validation Files
          description: >-
            A list containing the IDs of uploaded files that contain validation
            data.
        object:
          const: job
          default: job
          title: Object
        fine_tuned_model:
          type: string
          title: Fine Tuned Model
          description: >-
            The name of the fine-tuned model that is being created. The value
            will be `null` if the fine-tuning job is still running.
        integrations:
          items:
            $ref: '#/components/schemas/WandbIntegrationOut'
          type: array
          title: Integrations
          description: A list of integrations enabled for your fine-tuning job.
      required:
        - id
        - hyperparameters
        - model
        - status
        - job_type
        - created_at
        - modified_at
        - training_files
      title: JobOut
      type: object
    JobsOut:
      properties:
        data:
          default: []
          items:
            $ref: '#/components/schemas/JobOut'
          title: Data
          type: array
        object:
          const: list
          default: list
          title: Object
      title: JobsOut
      type: object
    TrainingParameters:
      description: The fine-tuning hyperparameter settings used in a fine-tune job.
      properties:
        training_steps:
          minimum: 1
          title: Training Steps
          type: integer
          description: >
            The number of training steps to perform. A training step refers to

            a single update of the model weights during the fine-tuning process.

            This update is typically calculated using a batch of samples from
            the

            training dataset.
        learning_rate:
          default: 0.0001
          maximum: 1
          minimum: 1.e-8
          title: Learning Rate
          type: number
          description: >
            A parameter describing how much to adjust the pre-trained model's
            weights

            in response to the estimated error each time the weights are updated
            during

            the fine-tuning process.
      required:
        - training_steps
      title: TrainingParameters
      type: object
    WandbIntegrationOut:
      properties:
        type:
          const: wandb
          default: wandb
          title: Type
        project:
          title: Project
          type: string
          description: The name of the project that the new run will be created under.
        name:
          type: string
          title: Name
          description: >-
            A display name to set for the run. If not set, will use the job ID
            as the name.
      required:
        - project
      title: WandbIntegrationOut
      type: object
    JobIn:
      properties:
        model:
          $ref: '#/components/schemas/FineTuneableModel'
        training_files:
          items:
            format: uuid
            type: string
          description: >-
            A list containing the IDs of uploaded files that contain training
            data.
          minItems: 1
          title: Training Files
          type: array
        validation_files:
          description: >
            A list containing the IDs of uploaded files that contain validation
            data.


            If you provide these files, the data is used to generate validation
            metrics

            periodically during fine-tuning. These metrics can be viewed in
            `checkpoints`

            when getting the status of a running fine-tuning job.


            The same data should not be present in both train and validation
            files.
          items:
            format: uuid
            type: string
          type: array
          title: Validation Files
        hyperparameters:
          $ref: '#/components/schemas/TrainingParameters'
        suffix:
          maxLength: 18
          type: string
          title: Suffix
          description: |
            A string that will be added to your fine-tuning model name.
            For example, a suffix of "my-great-model" would produce a model
            name like `ft:open-mistral-7b:my-great-model:xxx...`
        integrations:
          description: A list of integrations to enable for your fine-tuning job.
          items:
            $ref: '#/components/schemas/WandbIntegration'
          type: array
          uniqueItems: true
          title: Integrations
      required:
        - model
        - training_files
        - hyperparameters
      title: JobIn
      type: object
    WandbIntegration:
      properties:
        type:
          const: wandb
          default: wandb
          title: Type
        project:
          title: Project
          type: string
          description: The name of the project that the new run will be created under.
        name:
          type: string
          title: Name
          description: >-
            A display name to set for the run. If not set, will use the job ID
            as the name.
        api_key:
          title: Api Key
          type: string
          description: The WandB API key to use for authentication.
      required:
        - project
        - api_key
      title: WandbIntegration
      type: object
    CheckpointOut:
      properties:
        metrics:
          $ref: '#/components/schemas/MetricOut'
        step_number:
          title: Step Number
          type: integer
          description: The step number that the checkpoint was created at.
        created_at:
          title: Created At
          type: integer
          description: The UNIX timestamp (in seconds) for when the checkpoint was created.
      required:
        - metrics
        - step_number
        - created_at
      title: CheckpointOut
      type: object
    DetailedJobOut:
      properties:
        id:
          format: uuid
          title: Id
          type: string
        hyperparameters:
          $ref: '#/components/schemas/TrainingParameters'
        model:
          $ref: '#/components/schemas/FineTuneableModel'
        status:
          enum:
            - QUEUED
            - STARTED
            - RUNNING
            - FAILED
            - SUCCESS
            - CANCELLED
            - CANCELLATION_REQUESTED
          title: Status
          type: string
          description: The current status of the fine-tuning job.
        job_type:
          title: Job Type
          type: string
          description: The type of job (`FT` for fine-tuning).
        created_at:
          title: Created At
          type: integer
          description: >-
            The UNIX timestamp (in seconds) for when the fine-tuning job was
            created.
        modified_at:
          title: Modified At
          type: integer
          description: >-
            The UNIX timestamp (in seconds) for when the fine-tuning job was
            last modified.
        training_files:
          items:
            format: uuid
            type: string
          title: Training Files
          type: array
          description: >-
            A list containing the IDs of uploaded files that contain training
            data.
        validation_files:
          items:
            format: uuid
            type: string
          type: array
          default: []
          title: Validation Files
          description: >-
            A list containing the IDs of uploaded files that contain validation
            data.
        object:
          const: job
          default: job
          title: Object
        fine_tuned_model:
          type: string
          title: Fine Tuned Model
          description: >-
            The name of the fine-tuned model that is being created. The value
            will be `null` if the fine-tuning job is still running.
        integrations:
          items:
            $ref: '#/components/schemas/WandbIntegrationOut'
          type: array
          title: Integrations
          description: A list of integrations enabled for your fine-tuning job.
        events:
          default: []
          items:
            $ref: '#/components/schemas/EventOut'
          title: Events
          type: array
          description: >
            Event items are created every time the status of a fine-tuning job
            changes.

            The timestamped list of all events is accessible here.
        checkpoints:
          default: []
          items:
            $ref: '#/components/schemas/CheckpointOut'
          title: Checkpoints
          type: array
      required:
        - id
        - hyperparameters
        - model
        - status
        - job_type
        - created_at
        - modified_at
        - training_files
      title: DetailedJobOut
      type: object
    EventOut:
      properties:
        name:
          title: Name
          type: string
          description: The name of the event.
        data:
          enum:
            - QUEUED
            - STARTED
            - RUNNING
            - FAILED
            - SUCCESS
            - CANCELLED
            - CANCELLATION_REQUESTED
          type: string
          title: Data
          description: The status of the fine-tuning job at the time of the event
        created_at:
          title: Created At
          type: integer
          description: The UNIX timestamp (in seconds) of the event.
      required:
        - name
        - created_at
      title: EventOut
      type: object
    MetricOut:
      properties:
        train_loss:
          type: number
          title: Train Loss
        valid_loss:
          type: number
          title: Valid Loss
        valid_mean_token_accuracy:
          type: number
          title: Valid Mean Token Accuracy
      title: MetricOut
      description: >
        Metrics at the step number during the fine-tuning job. Use these metrics
        to

        assess if the training is going smoothly (loss should decrease, token
        accuracy

        should increase).
      type: object
    UploadFileResponse:
      properties:
        id:
          format: uuid
          title: Id
          type: string
          example: 497f6eca-6276-4993-bfeb-53cbbbba6f09
        object:
          title: Object
          type: string
          example: test
        bytes:
          title: Bytes
          type: integer
          example: 13000
        created_at:
          title: Created At
          type: integer
          example: 1716963433
        filename:
          title: Filename
          type: string
          example: files_upload.jsonl
        purpose:
          title: Purpose
          type: string
          example: fine-tune
      required:
        - id
        - object
        - bytes
        - created_at
        - filename
        - purpose
      title: UploadFileResponse
      type: object
    FileSchema:
      properties:
        id:
          format: uuid
          title: Id
          type: string
          description: The file identifier, which can be referenced in the API endpoints
          example: d56b5e4f-16ae-4f07-be8e-b837aa10240f
        object:
          title: Object
          type: string
          description: The object type, which is always `file`.
          example: file
        bytes:
          title: Bytes
          type: integer
          description: The size of the file, in bytes.
          example: 1534119
        created_at:
          title: Created At
          type: integer
          description: The UNIX timestamp (in seconds) for when the file was created.
          example: 1716329302
        filename:
          title: Filename
          type: string
          description: The name of the file
          example: file_upload.jsonl
        purpose:
          title: Purpose
          type: string
          description: The intended purpose of the file. Only supports `fine-tune` for now.
          example: fine-tune
      required:
        - id
        - object
        - bytes
        - created_at
        - filename
        - purpose
      title: FileSchema
      type: object
    ListFilesResponse:
      properties:
        data:
          items:
            $ref: '#/components/schemas/FileSchema'
          title: Data
          type: array
        object:
          title: Object
          type: string
      required:
        - data
        - object
      title: ListFilesResponse
      type: object
    RetrieveFileResponse:
      properties:
        id:
          format: uuid
          title: Id
          type: string
        object:
          title: Object
          type: string
        bytes:
          title: Bytes
          type: integer
        created_at:
          title: Created At
          type: integer
        filename:
          title: Filename
          type: string
        purpose:
          title: Purpose
          type: string
      required:
        - id
        - object
        - bytes
        - created_at
        - filename
        - purpose
      title: RetrieveFileResponse
      type: object
    DeleteFileResponse:
      properties:
        id:
          format: uuid
          title: Id
          type: string
        object:
          title: Object
          type: string
        deleted:
          title: Deleted
          type: boolean
      required:
        - id
        - object
        - deleted
      title: DeleteFileResponse
      type: object
    FIMCompletionResponse:
      type: object
      properties:
        id:
          type: string
          example: 5b35cc2e69bf4ba9a11373ee1f1937f8
        object:
          type: string
          example: chat.completion
        created:
          type: integer
          example: 1702256327
        model:
          type: string
          example: codestral-latest
        choices:
          type: array
          items:
            type: object
            required:
              - index
              - text
              - finish_reason
            properties:
              index:
                type: integer
                example: 0
              message:
                type: object
                properties:
                  role:
                    type: string
                    enum:
                      - user
                      - assistant
                    example: assistant
                  content:
                    type: string
                    example: '" add(a,b):"'
              finish_reason:
                type: string
                enum:
                  - stop
                  - length
                  - model_length
                  - error
                example: stop
        usage:
          type: object
          properties:
            prompt_tokens:
              type: integer
              example: 8
            completion_tokens:
              type: integer
              example: 9
            total_tokens:
              type: integer
              example: 17
          required:
            - prompt_tokens
            - completion_tokens
            - total_tokens
    JobMetadata:
      type: object
      title: JobMetadata
      properties:
        training_steps:
          type: integer
          description: >
            The number of training steps to perform. A training step refers to a
            single update of the model weights during the fine-tuning process.
            This update is typically calculated using a batch of samples from
            the training dataset.
          name: Training steps
          example: 10
        train_tokens_per_step:
          type: integer
          description: The number of tokens consumed by one training step.
          name: Training tokens per step
          example: 131072
        data_tokens:
          type: integer
          description: The total number of tokens in the training dataset.
          example: 305375
        train_tokens:
          type: integer
          description: The total number of tokens used during the fine-tuning process.
          example: 1310720
        epochs:
          type: float
          description: The number of complete passes through the entire training dataset.
          example: 4.2922
        expected_duration_seconds:
          type: integer
          description: >-
            The approximated time (in seconds) for the fine-tuning process to
            complete.
          example: 220
    HTTPValidationError:
      properties:
        detail:
          items:
            $ref: '#/components/schemas/ValidationError'
          type: array
          title: Detail
      type: object
      title: HTTPValidationError
    ValidationError:
      properties:
        loc:
          items:
            anyOf:
              - type: string
              - type: integer
          type: array
          title: Location
        msg:
          type: string
          title: Message
        type:
          type: string
          title: Error Type
      type: object
      required:
        - loc
        - msg
        - type
      title: ValidationError
