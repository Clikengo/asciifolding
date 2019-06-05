use criterion::black_box;
use criterion::Criterion;
use criterion::ParameterizedBenchmark as PB;
use criterion::Throughput;
use criterion::{criterion_group, criterion_main};

use asciifolding::{fold_char_to_ascii, fold_to_ascii, fold_to_ascii_buffer};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "fold",
        PB::new(
            "simple",
            move |b, txt| b.iter(|| black_box(fold_to_ascii(black_box(txt)))),
            vec![
                /*"Lôrém îpsum dolôr sît amët, pér nè vîtupëràta scriptôrem, \
                quî nô çônvênire quâlïsqùë. At albucius rêcûsabo mea, pœpulô \
                rèformidans dïssëntîunt te êst, sit an fallî îgnota vîtuperatorïbus. \
                Fàçètè œportéré expétënda eum cu, ne usù vîtâè impedit scàévola. \
                Iusto glôrïatur ea çum, mëï ei dùis detèrruîsset, quidam albûcius \
                réprimîqùê ad has. Eam pêrsius tibiqùë argùméntûm et, mèa qùandœ \
                officïîs intéllègat ei, àn mèï dicit ôporteat àntïopam. Quôd vélit \
                accommôdâre vix ût, cû lègerë èffïçiendi qùï.",*/
                "Lorem ipsum dolor sit amet, te qui esse ubique. Sed ullum melius \
                 utamur ex, in usu facer paulo solet, ornatus recusabo an mel. \
                 Nam iudico repudiandae delicatissimi ne, accusata tractatos cu \
                 qui, dicunt expetenda ne mei. An nam illum inciderint, id nihil \
                 voluptatum nec, nec et utamur utroque molestiae. Vel erat soluta \
                 definitiones in, an impedit facilisis assueverit mel. Nemore \
                 scripta te sit, quo clita iracundia no, etiam inimicus pri ea.",
                //"Lôrém îpsum dolôr sit amet, te qui esse ubique.",
                //"Lôrém îpsum dolôr sît amët, te qui esse ubique.",
            ],
        )
        .with_function("simple lc", move |b, txt| {
            b.iter(|| black_box(fold_to_ascii(black_box(txt)).to_lowercase()))
        })
        .with_function("lc", move |b, txt| {
            b.iter(|| black_box(black_box(txt).to_lowercase()))
        })
        .with_function("reuse", move |b, txt| {
            let mut output = String::with_capacity(txt.len());
            b.iter(|| {
                output.clear();
                fold_to_ascii_buffer(black_box(txt), &mut output);
            })
        })
        .with_function("reuse lc", move |b, txt| {
            let mut output = String::with_capacity(txt.len());
            b.iter(|| {
                output.clear();
                fold_to_ascii_buffer(black_box(txt), &mut output);
                output.to_lowercase()
            })
        })
        .with_function("reuse iter", move |b, txt| {
            let mut output = String::with_capacity(txt.len());
            b.iter(|| {
                output.clear();
                output.extend(txt.chars().flat_map(fold_char_to_ascii));
            })
        })
        .with_function("reuse iter lc", move |b, txt| {
            let mut output = String::with_capacity(txt.len());
            b.iter(|| {
                output.clear();
                output.extend(
                    txt.chars()
                        .flat_map(fold_char_to_ascii)
                        .flat_map(char::to_lowercase),
                );
            })
        })
        .throughput(|txt| Throughput::Bytes(txt.len() as u32)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
