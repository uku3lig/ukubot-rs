use crate::command::UkubotCommand;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::Permissions;

pub struct RatioCommand;

static RATIO: Lazy<Vec<String>> = Lazy::new(|| {
    let s = "what is this + L + ratio + wrong + get a job + unfunny + you fell off + never liked you anyway + cope + you can't catch this ratio + why about you talk with real peoples + I don't care about your opinion + genshin player + put some thought into what you're going to do with that + au dodo + go to bed + yes, i'm taller than you + I win + conversation over + :) + you don’t know 2 + 2 with yo head + you are going to my cringe compilation + try again loser + rickrolled + no lifer + go ahead whine about it + eat paper + you lose + your problem + no one cares + log off + not okay + glhf + problematic + dog water + you look like a wall + you're a confused block of cheese + slight_smile + aired + cringe again + mad cuz bad + irrelevant + deal with it + screencapped your bio + jealous + i'll be right back + ask deez + ez clap + straight cash + idgaf + ratio again + stay mad + stay pressed + cancelled + done for + don't give a damn + get a job + get blocked + glace à la pistache + rip bozo + no + ok + ok boomer + France-Dijon + Oignon rouge + Méthode Roux + coefficient de raideur + le rap français + la myopie + acide hyaluronique + eau pétillante + chicken louisiane steackhouse + france tv + Rust + plaque tectonique + la troisième loi de Newton+ Scwheps agrume + legrand electronique + BDW TP5 + elodie + valise + méthode d'orthonormalisation de gram-schmidt + hannah montana + les simsons le film + actifry + pic pic alcool et drame + bourre bourre jfais un dram + OM-PSG + 30 mg de sucre en poudre + nescafé + chignon+ Rubik's cube + leo in the baignoire + Le pivot de Gauss + L'aventurier + matrice échelonnée + mocap + l'abonnement velov à 16€ + gdb + module image + la belote + Level'Up + mister mv + stabilo boss + sauce algérienne + ligma + pikachu + Chargeur usb C + métro c + inflation + Pablo + pyjama dinausore + Jonnhy Halliday à Bercy + + QWERTY + télésiège + fraude fiscale + escalope milanaise + les lacs du côneMara + damnation + Chocolat Viennois + raclette + Intégrale de Riemann + Macbook Air + de a u 4 + hubble telescope + sigma grindset + distributivité simple + LIFRW + Chaise pliante + inégalité de Cauchy Schwartz + no polypoints + eduroam + grapic + le duke + macdo charpennes+ Take the L + rip bozo + auttgames + développement limité + ucbl portail + pdt + metro C + Eddy Malou + Mimir moment + feur +";
    s.split(" + ").map(String::from).collect()
});

#[serenity::async_trait]
impl UkubotCommand for RatioCommand {
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.name("ratio").description("FEUR")
    }

    async fn on_command(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> anyhow::Result<()> {
        let ratio = RATIO
            .choose_multiple(&mut rand::thread_rng(), 20)
            .cloned()
            .collect::<Vec<_>>()
            .join(" + ");

        interaction
            .create_interaction_response(&ctx.http, |r| {
                r.interaction_response_data(|d| d.content(ratio))
            })
            .await?;

        Ok(())
    }
}

pub struct EchoCommand;

#[serenity::async_trait]
impl UkubotCommand for EchoCommand {
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("echo")
            .description("say things")
            .create_option(|o| {
                o.name("text")
                    .description("the thing to say")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
            .default_member_permissions(Permissions::ADMINISTRATOR)
    }

    async fn on_command(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> anyhow::Result<()> {
        let text = interaction
            .data
            .options
            .iter()
            .find(|o| o.name == "text")
            .ok_or(anyhow!("Could not find parameter text"))?
            .value
            .as_ref()
            .ok_or(anyhow!("Could not find value for parameter text"))?
            .as_str()
            .unwrap();

        interaction.channel_id.say(&ctx.http, text).await?;

        interaction
            .create_interaction_response(&ctx.http, |r| {
                r.interaction_response_data(|d| d.content("sent!").ephemeral(true))
            })
            .await?;

        Ok(())
    }
}
