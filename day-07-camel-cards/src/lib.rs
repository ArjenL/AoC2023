use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();
    part1(ctx);
    ctx.update_timer(Ctx::PART1);

    part2(ctx);
    ctx.update_timer(Ctx::PART2);
}

fn part1(ctx: &mut Ctx) {
    let answer = 42;

    outputln!(ctx, "part 1: {}", answer);

}

fn part2(ctx: &mut Ctx) {
    let answer = 42;
    outputln!(ctx, "part 2: {}", answer);
}
