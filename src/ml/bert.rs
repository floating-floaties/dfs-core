
pub mod ul {
    use rust_bert::pipelines::question_answering::{Answer, QaInput, QuestionAnsweringModel};
    use rust_bert::pipelines::sequence_classification::Label;
    use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;

    pub(crate) fn intent(sentences: &[&str], labels: &[&str]) -> Vec<Vec<Label>> {
        let sequence_classification_model = ZeroShotClassificationModel::new(Default::default()).unwrap();

        sequence_classification_model.predict_multilabel(
            sentences,
            labels,
            Some(Box::new(|label: &str| {
                format!("This example is about {}.", label)
            })),
            128,
        )
    }


    pub(crate) fn qa(qas: &[QaInput]) -> Vec<Vec<Answer>> {

        //    Set-up Question Answering model
        let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();

        //    Define input
        // let mut squad_path = std::path::PathBuf::from(".");
        // squad_path.push("dev-v2.0.json");
        // let mut qa_inputs = squad_processor(squad_path);
        //
        // // qa_inputs.insert(0, QaInput {
        // //     question: "Who were the Normans?".into(),
        // //     context: "The Normans (Norman: Nourmands; French: Normands; Latin: Normanni) were the people who in the 10th and 11th centuries gave their name to Normandy, a region in France. They were descended from Norse (\"Norman\" comes from \"Norseman\") raiders and pirates from Denmark, Iceland and Norway who, under their leader Rollo, agreed to swear fealty to King Charles III of West Francia. Through generations of assimilation and mixing with the native Frankish and Roman-Gaulish populations, their descendants would gradually merge with the Carolingian-based cultures of West Francia. The distinct cultural and ethnic identity of the Normans emerged initially in the first half of the 10th century, and it continued to evolve over the succeeding centuries.".into()
        // // });
        //    Get answer
        let answers = qa_model.predict(qas, 3, 64);
        println!("Sample answer: {:?}", answers.first().unwrap());
        println!("{}", answers.len());
        answers
    }
}

#[cfg(test)]
mod bert_tests {
    use rust_bert::pipelines::question_answering::QaInput;
    use crate::ml::bert::ul::*;

    #[test]
    fn qa_test() {
        let qas = &[
            QaInput {
                question: "What is the main language of Puerto Rico?".into(),
                context: "Puerto Rico officially the Commonwealth of Puerto Rico, is a Caribbean island and unincorporated territory of the United States. It is located in the northeast Caribbean Sea, approximately 1,000 miles (1,600 km) southeast of Miami, Florida, between the Dominican Republic and the U.S. Virgin Islands, and includes the eponymous main island and several smaller islands, such as Mona, Culebra, and Vieques. It has roughly 3.2 million residents, and its capital and most populous city is San Juan.[10] Spanish and English are the official languages of the executive branch of government, though Spanish predominates. ".into()
            }
        ];
        let answers = qa(qas);
        println!("answers={answers:?}")
    }

    #[test]
    fn intent_test() {

        let input_sentence = &["How do I pay?", "I am locked out!"];
        let candidate_labels = &["login issue", "billing", "contact support"];

        let result = intent(input_sentence, candidate_labels);
        println!("{result:?}");
    }
}
