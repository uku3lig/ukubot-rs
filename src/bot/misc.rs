use crate::{config::GuildConfig, Context};
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use poise::serenity_prelude::{self as serenity, channel};
use rand::seq::SliceRandom;

static RATIO: Lazy<Vec<String>> = Lazy::new(|| {
    let s = "what is this + L + ratio + wrong + get a job + unfunny + you fell off + never liked you anyway + cope + you can't catch this ratio + why about you talk with real peoples + I don't care about your opinion + genshin player + put some thought into what you're going to do with that + au dodo + go to bed + yes, i'm taller than you + I win + conversation over + :) + you don’t know 2 + 2 with yo head + you are going to my cringe compilation + try again loser + rickrolled + no lifer + go ahead whine about it + eat paper + you lose + your problem + no one cares + log off + not okay + glhf + problematic + dog water + you look like a wall + you're a confused block of cheese + slight_smile + aired + cringe again + mad cuz bad + irrelevant + deal with it + screencapped your bio + jealous + i'll be right back + ask deez + ez clap + straight cash + idgaf + ratio again + stay mad + stay pressed + cancelled + done for + don't give a damn + get a job + get blocked + glace à la pistache + rip bozo + no + ok + ok boomer + France-Dijon + Oignon rouge + Méthode Roux + coefficient de raideur + le rap français + la myopie + acide hyaluronique + eau pétillante + chicken louisiane steackhouse + france tv + Rust + plaque tectonique + la troisième loi de Newton+ Scwheps agrume + legrand electronique + BDW TP5 + elodie + valise + méthode d'orthonormalisation de gram-schmidt + hannah montana + les simsons le film + actifry + pic pic alcool et drame + bourre bourre jfais un dram + OM-PSG + 30 mg de sucre en poudre + nescafé + chignon+ Rubik's cube + leo in the baignoire + Le pivot de Gauss + L'aventurier + matrice échelonnée + mocap + l'abonnement velov à 16€ + gdb + module image + la belote + Level'Up + mister mv + stabilo boss + sauce algérienne + ligma + pikachu + Chargeur usb C + métro c + inflation + Pablo + pyjama dinausore + Jonnhy Halliday à Bercy + + QWERTY + télésiège + fraude fiscale + escalope milanaise + les lacs du côneMara + damnation + Chocolat Viennois + raclette + Intégrale de Riemann + Macbook Air + de a u 4 + hubble telescope + sigma grindset + distributivité simple + LIFRW + Chaise pliante + inégalité de Cauchy Schwartz + no polypoints + eduroam + grapic + le duke + macdo charpennes+ Take the L + rip bozo + auttgames + développement limité + ucbl portail + pdt + metro C + Eddy Malou + Mimir moment + feur +";
    s.split(" + ").map(String::from).collect()
});

/// FEUR
#[poise::command(slash_command)]
pub async fn ratio(ctx: Context<'_>) -> Result<()> {
    let ratio = RATIO
        .choose_multiple(&mut rand::thread_rng(), 20)
        .cloned()
        .collect::<Vec<_>>()
        .join(" + ");

    ctx.reply(ratio).await?;

    Ok(())
}

/// say things
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "the thing to say"] text: String,
) -> Result<()> {
    ctx.channel_id()
        .send_message(&ctx, |m| m.content(text))
        .await?;

    ctx.send(|b| b.content("done").ephemeral(true)).await?;

    Ok(())
}

/// configures the bot to your linkings
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn config(
    ctx: Context<'_>,
    #[description = "whether or not requests are open"] requests_open: Option<bool>,
    #[description = "the text channel where forms are sent"]
    #[channel_types("Text")]
    requests_channel: Option<serenity::GuildChannel>,
    #[description = "the category where tickets are created"]
    #[channel_types("Category")] // FIXME poise/serenity issue waaaah
    ticket_category: Option<serenity::GuildChannel>,
    #[description = "the category where closed tickets are moved"]
    #[channel_types("Category")]
    closed_category: Option<serenity::GuildChannel>,
    #[description = "the text channel where finished tickets are sent"]
    #[channel_types("Text")]
    finished_channel: Option<serenity::GuildChannel>,
) -> Result<()> {
    let guild_id = ctx
        .guild_id()
        .ok_or(anyhow!("command must be run in a guild"))?;

    let mut config = GuildConfig::get(guild_id);

    if let Some(requests_open) = requests_open {
        config.requests_open = requests_open;
    }

    if let Some(requests_channel) = requests_channel {
        if requests_channel.kind == serenity::ChannelType::Text {
            config.requests_channel = requests_channel.id;
        } else {
            ctx.send(|m| {
                m.content("requests channel must be a text channel")
                    .ephemeral(true)
            })
            .await?;
            return Ok(());
        }
    }

    if let Some(ticket_category) = ticket_category {
        if ticket_category.kind == serenity::ChannelType::Category {
            config.ticket_category = ticket_category.id;
        } else {
            ctx.send(|m| {
                m.content("ticket category must be a category channel")
                    .ephemeral(true)
            })
            .await?;
            return Ok(());
        }
    }

    if let Some(closed_category) = closed_category {
        if closed_category.kind == serenity::ChannelType::Category {
            config.closed_category = closed_category.id;
        } else {
            ctx.send(|m| {
                m.content("closed category must be a category channel")
                    .ephemeral(true)
            })
            .await?;
            return Ok(());
        }
    }

    if let Some(finished_channel) = finished_channel {
        if finished_channel.kind == serenity::ChannelType::Text {
            config.finished_channel = finished_channel.id;
        } else {
            ctx.send(|m| {
                m.content("finished channel must be a text channel")
                    .ephemeral(true)
            })
            .await?;
            return Ok(());
        }
    }

    config.save(guild_id)?;

    ctx.send(|m| m.content("successfully updated config").ephemeral(true))
        .await?;

    Ok(())
}
