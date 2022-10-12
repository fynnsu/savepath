#!/bin/zsh
echo cb () {{
            CB_CMD=$(
                clipboard $@
            ) && eval $CB_CMD;
        }}



        # '''.format(
        #     name=alias_name,
        #     argument_placeholder=ARGUMENT_PLACEHOLDER,
        #     alter_history=('test -n "$TF_CMD" && print -s $TF_CMD'
        #                    if settings.alter_history else ''))