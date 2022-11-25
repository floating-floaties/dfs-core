use rust_bert::gpt_neo::{GptNeoConfigResources, GptNeoMergesResources, GptNeoModelResources, GptNeoVocabResources};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::sentence_embeddings::builder::Remote;
use rust_bert::pipelines::text_generation::{TextGenerationConfig, TextGenerationModel};
use rust_bert::resources::RemoteResource;

#[test]
fn it_works() {
    println!("Loading model resosurce");
    let model_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoModelResources::GPT_NEO_2_7B,
    ));

    println!("Loading config resosurce");
    let  config_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoConfigResources::GPT_NEO_2_7B,
    ));
    //
    println!("Loading vocab resosurce");
    let vocab_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoVocabResources::GPT_NEO_2_7B,
    ));

    println!("Loading merges resosurce");
    let merges_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoMergesResources::GPT_NEO_2_7B,
    ));
    //
    println!("creating config");
    let generate_config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        num_beams: 5,
        no_repeat_ngram_size: 2,
        max_length: 100,
        ..Default::default()
    };

    println!("Creating model");
    let model = TextGenerationModel::new(
        generate_config
    ).expect("fail to create kmode;");

    print!(">> ");
    let line = "Set sail!/I will be king of the pirates!".to_string();
    let split: Vec<&str> = line.split('/').collect();
    let slc = split.as_slice();
    let output = model.generate(&slc[1..], Some(slc[0]));

    for sentence in output {
        println!(">> {sentence}");
    }

}