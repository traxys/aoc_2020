use crate::DayContext;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(expr_parser, "/problems/day18.rs");

type Input = Vec<String>;

pub fn part_1(exprs: Input) -> color_eyre::Result<String> {
    let sum: u64 = exprs
        .iter()
        .map(|s| expr_parser::NoPrecExprParser::new().parse(s).unwrap())
        .sum();
    Ok(format!("Sum of all exprs is: {}", sum))
}

pub fn part_2(exprs: Input) -> color_eyre::Result<String> {
    let sum: u64 = exprs
        .iter()
        .map(|s| expr_parser::InvPrecExprParser::new().parse(s).unwrap())
        .sum();
    Ok(format!("Sum of all exprs is: {}", sum))
}

#[cfg(test)]
mod test {
    use super::expr_parser;

    #[test]
    fn test_simple_exprs() {
        let expr = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(
            expr_parser::NoPrecExprParser::new().parse(&expr).unwrap(),
            71
        );

        let expr = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(
            expr_parser::NoPrecExprParser::new().parse(&expr).unwrap(),
            51
        );
    }

    #[test]
    fn inv_prec() {
        let expr = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(
            expr_parser::InvPrecExprParser::new().parse(&expr).unwrap(),
            231
        );

        let expr = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(
            expr_parser::InvPrecExprParser::new().parse(&expr).unwrap(),
            51
        );
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    context.parse_lines(|s| Ok(s.to_string()))
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}
