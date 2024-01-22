use bitflags::bitflags;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let xmltype = args.xmltype.to_lowercase();
    let value = args.value;
    

    match xmltype.as_str() {
        "skill" => skill_check(value),
        "chapter" => chapter_check(value),
        _ => println!("Unknown XML type")
    }

    }

    fn skill_check(value: u64) {
        let inputflags = &SkillFlags::from_bits_truncate(value);
        if SkillFlags::is_empty(inputflags) == true {
            println!("No flags found.");
        } else {
            let flagvalue = SkillFlags::bits(inputflags).to_string();
            println!("Input Flag Value: {}", flagvalue);
            let flagfound = inputflags.iter_names();
            for(name, _) in flagfound {
                let ffvalue = &SkillFlags::from_name(name).unwrap();
                let ffvalue = SkillFlags::bits(ffvalue);
                println!("Flag Found: {}, {}", name, ffvalue);
            }
            
        }
    }

    fn chapter_check(value: u64) {
        let inputflags = &ChapterFlags::from_bits_truncate(value);
        if ChapterFlags::is_empty(inputflags) == true {
            println!("No flags found.");
        } else {
            let flagvalue = ChapterFlags::bits(inputflags).to_string();
            println!("Flag Value: {}", flagvalue);
            let flagfound = inputflags.iter_names();
            for(name, _) in flagfound {
                let ffvalue = &ChapterFlags::from_name(name).unwrap();
                let ffvalue = ChapterFlags::bits(ffvalue);
                println!("Flag Found: {}, {}", name, ffvalue);
            }
            
        }
    }

bitflags!{
    struct SkillFlags: u64 {
        const Invisible = 1;
        const EngageAttack = 2;
        const EngageCharge = 4;
        const EngageLink = 8;
        const EngageWait = 16;
        const EngageSummon = 32;
        const IgnoreEngageAttacking = 64;
        const IgnoreNoEngageAttacking = 128;
        const EnableChaining = 256;
        const EnableDestory = 512;
        const EnableCannon = 1024;
        const EnableRod = 2048;
        const IgnoreAlone = 4096;
        const IgnoreMultiAttacking = 8192;
        const IgnoreTraining = 16384;
        const IgnoreTraial = 32768;
        const IgnoreSimulation = 65536;
        const ExclusiveDance = 131072;
        const RevengeAutoEquip = 262144;
        const SwapOrder = 524288;
        const InterruptOrder = 1048576;
        const ContinueBattle = 2097152;
        const ForceLateOrder = 4194304;
        const EachSupport = 8388608;
        const Reactable = 16777216;
        const Remagicable = 33554432;
        const BeforeMove = 67108864;
        const AllowChainAttack = 134217728;
        const AllowChainGuard = 268435456;
        const AllowEngageGuard = 546870912;
        const ForceChainAttack = 1073741824;
        const JoinChainAttack = 2147483648;
        const RangeReliance = 4294967296;
        const PickupReliance = 8589934592;
        const MoveCostFree = 17179869184;
        const MoveEnemyPass = 34359738368;
        const ResetDisorder = 68719476736;
        const ItemHealAround = 137438953472;
        const ItemHealGive = 274877906944;
        const SelfHealRod = 549755813888;
        const OnlyRecvoerRod = 1099511627776;
        const DecayEnchance = 2199023255552;
        const SubEngageCountLimit = 4398046511104;
        const ReverseCount = 8796093022208;
        const ReCooking = 17592186044416;
        const BasisSkill = 35184372088832;
        const Unstoppable = 70368744177664;
        const HideChangeGod = 140737488355328;
        const OverExpChange = 281474976710656;
        const MoveFly = 562949953421312;
        const ViewRestriction = 1125899906842624;
        const HasIconBmap = 9007199254740992;
        const HasContract = 18014398509481984;
        const HauntChainAttack = 36028797018963968;
        const HasRootCommand = 72057594037927936;
        const HasZOC = 144115188075855872;
        const HasWork = 288230376151711744;
        const HasVision = 576460752303423488;
        const NotCondition = 1152921504606846976;
        const HasCondition = 2305843009213693952;
        const HasEnhance = 4611686018427387904;
    }

    struct ChapterFlags: u64 {
        const Sally = 1;
        const CanBack = 2;
        const Sight = 4;
        const Kizuna = 8;
        const Hub = 16;
        const Gmap = 32;
        const Continue = 64;
        const Serious = 128;
        const Casual = 256;
        const Challenge = 512;
        const Relay = 1024;
        const Versus = 2048;
        const TestMap = 4096;
        const Opposition = 8192;
        const HighRankItem = 16384;
        const CanSlope = 32768;
        const SideStory= 1073741824;
    }
    
}


#[derive(Parser)]
struct Cli {
    xmltype: String,
    value: u64,
}